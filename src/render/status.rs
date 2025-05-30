use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::state::AppState;
use crate::ui::status::status_line;
use crate::ui::animate::breath_style;
use crate::render::favorites::render_favorites_dock;

pub fn render_status_bar<B: Backend>(f: &mut Frame<B>, area: Rect, state: &mut AppState) {
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    if let Some(last) = state.status_message_last_updated {
        if last.elapsed() > Duration::from_secs(4) {
            state.status_message.clear();
            state.status_message_last_updated = None;
        }
    }

    let default_status = status_line(state);
    let display_string = if state.status_message.is_empty() {
        default_status
    } else {
        state.status_message.clone()
    };

    let tick = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        / 300;
    let hearts = ["💚", "💛", "❤️", "💙", "🖤"];
    let heart = hearts[((tick / 10) as usize) % hearts.len()];
    let heart_style = breath_style(Color::White, tick as u64);
    let spans = Spans::from(vec![
        Span::styled(heart, heart_style),
        Span::raw(" "),
        Span::raw(display_string),
    ]);

    let block = Block::default().borders(Borders::ALL).title("Status");
    let content = Paragraph::new(spans).style(Style::default());
    f.render_widget(block, area);
    let inner_width = area.width.saturating_sub(2);
    f.render_widget(content, Rect::new(area.x + 1, area.y + 1, inner_width, 1));
    render_favorites_dock(f, area, state);
}
