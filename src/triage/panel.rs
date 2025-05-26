use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::text::{Line, Span};
use crate::state::AppState;
use crate::triage::logic::TriageSource;

pub fn render_triage_panel<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    let mut lines = Vec::new();
    for entry in &state.triage_entries {
        if entry.archived { continue; }
        let mut style = Style::default();
        if entry.resolved {
            style = style.fg(Color::DarkGray);
        }
        let src = match entry.source {
            TriageSource::Zen => "Zen",
            TriageSource::Gemx => "GemX",
            TriageSource::Spotlight => "Spotlight",
        };
        lines.push(Line::from(vec![Span::styled(format!("[{}] {}", entry.id, src), style), Span::raw(" "), Span::styled(&entry.text, style)]));
        lines.push(Line::from(Span::styled("[r]esolve [d]ismiss [a]rchive", Style::default().fg(Color::Blue))));
        lines.push(Line::from(""));
    }
    if lines.is_empty() {
        lines.push(Line::from("No triage entries"));
    }
    let para = Paragraph::new(lines)
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(para, area);
}
