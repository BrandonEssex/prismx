use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use crate::state::AppState;
use crate::inbox::{TaskStatus, Priority};

pub fn draw_triage_view(f: &mut Frame, state: &AppState) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
        .split(size);

    let header = Paragraph::new("📥 Triage Inbox — Use arrows to navigate, Ctrl+D to archive")
        .block(Block::default().borders(Borders::ALL).title("Inbox Triage"))
        .style(Style::default().add_modifier(Modifier::BOLD));
    f.render_widget(header, chunks[0]);

    let inbox_tasks: Vec<_> = state.inbox.list_by_status(TaskStatus::Inbox);

    let items: Vec<ListItem> = inbox_tasks
        .iter()
        .map(|task| {
            let mut line = format!("• [{}] {}", format!("{:?}", task.priority), task.title);

            if let Some(shard) = &task.shard {
                line.push_str(&format!(" | Shard: {}", shard));
            }
            if !task.tags.is_empty() {
                line.push_str(&format!(" | Tags: {}", task.tags.join(", ")));
            }
            if let Some(owner) = &task.assigned_to {
                line.push_str(&format!(" | Assigned: {}", owner));
            }

            ListItem::new(Line::from(vec![Span::raw(line)]))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Inbox Tasks"))
        .highlight_symbol(">> ");

    f.render_widget(list, chunks[1]);
}