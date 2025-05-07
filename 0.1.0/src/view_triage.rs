use crate::inbox::{InboxTask, TaskStatus};
use crate::state::AppState;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn draw_triage_view<B: Backend>(frame: &mut Frame<B>, state: &AppState) {
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
            let mut spans = vec![
                Span::raw(format!("• [{}] ", task.priority.to_string())),
                Span::styled(&task.title, Style::default().add_modifier(Modifier::BOLD)),
            ];

            if let Some(ref shard) = task.shard {
                spans.push(Span::raw(format!(" | Shard: {}", shard)));
            }

            if !task.tags.is_empty() {
                spans.push(Span::raw(format!(" | Tags: {}", task.tags.join(", "))));
            }

            if let Some(ref owner) = task.assigned_to {
                spans.push(Span::raw(format!(" | Assigned: {}", owner)));
            }

            ListItem::new(Spans::from(spans))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Inbox Tasks"))
        .highlight_symbol(">> ");

    frame.render_widget(list, chunks[1]);
}