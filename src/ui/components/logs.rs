use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::state::AppState;

const MAX_LINES: usize = 100;

pub fn render_logs<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    use std::fs;

    let data = fs::read_to_string("logs/prismx.log").unwrap_or_default();
    let mut lines: Vec<String> = data.lines().map(|l| l.to_string()).collect();

    if lines.len() > MAX_LINES {
        lines = lines.split_off(lines.len() - MAX_LINES);
    }

    let offset = state.logs_scroll.min(lines.len());
    let start = lines.len().saturating_sub(offset + MAX_LINES);
    let end = lines.len().saturating_sub(offset);
    let visible = &lines[start..end];

    let filter = state.logs_filter.to_lowercase();
    let items: Vec<ListItem> = visible
        .iter()
        .map(|l| {
            if !filter.is_empty() && l.to_lowercase().contains(&filter) {
                ListItem::new(Span::styled(l.clone(), Style::default().fg(Color::Yellow)))
            } else {
                ListItem::new(Span::raw(l.clone()))
            }
        })
        .collect();

    let width = area.width.min(80);
    let height = area.height.min(20).max(5);
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;

    let block = Block::default().title("Logs").borders(Borders::ALL);
    f.render_widget(block, Rect::new(x, y, width, height));

    let filter_area = Rect::new(x + 1, y + 1, width - 2, 1);
    f.render_widget(
        Paragraph::new(format!("Filter: {}", state.logs_filter)),
        filter_area,
    );

    let list_area = Rect::new(x + 1, y + 2, width - 2, height - 3);
    let list = List::new(items);
    f.render_widget(list, list_area);
}
