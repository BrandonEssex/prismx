use ratatui::{backend::Backend, style::{Color, Style}, widgets::{Block, Borders, Paragraph}, text::{Span, Line}, Frame};
use crate::ui::layout::Rect;

use crate::state::{AppState, TriageSummary};
use crate::ui::animate::shimmer;

pub fn render_triage_status<B: Backend>(f: &mut Frame<B>, area: Rect, state: &mut AppState) {
    let metrics: &mut TriageSummary = &mut state.triage_summary;
    let tick = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() / 300;

    let mut style = Style::default().fg(Color::Cyan);
    if metrics.highlight_frames > 0 {
        style = shimmer(Color::Cyan, tick as u64);
        metrics.highlight_frames -= 1;
    }

    let summary_text = format!("[ #NOW: {} ] [ #TRITON: {} ] [ #DONE: {} ]", metrics.now, metrics.triton, metrics.done);
    let mut spans = vec![Span::styled(summary_text, style)];

    if let Some(ref act) = metrics.last_action {
        spans.push(Span::raw(" "));
        spans.push(Span::styled(act.as_str(), Style::default().fg(Color::Yellow)));
    }

    let para = Paragraph::new(Line::from(spans))
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(para, area);
}
