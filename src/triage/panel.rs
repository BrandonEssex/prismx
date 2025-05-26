use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::text::{Line, Span};

use crate::state::AppState;
use crate::triage::logic::{TriageSource, tag_counts};
use crate::beamx::render_full_border;

pub fn render_triage_panel<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    let style = state.beam_style_for_mode("triage");

    let mut lines = Vec::new();

    let (now_count, triton_count, done_count) = tag_counts(state);
    let summary_style = Style::default().fg(Color::Cyan).add_modifier(Modifier::DIM);
    lines.push(Line::from(Span::styled(
        format!("[ #NOW: {} ] [ #TRITON: {} ] [ #DONE: {} ]", now_count, triton_count, done_count),
        summary_style,
    )));
    lines.push(Line::from(""));
    for entry in &state.triage_entries {
        if entry.archived { continue; }

        let mut entry_style = Style::default();
        if entry.resolved {
            entry_style = entry_style.fg(Color::DarkGray);
        }

        let src = match entry.source {
            TriageSource::Zen => "Zen",
            TriageSource::Gemx => "GemX",
            TriageSource::Spotlight => "Spotlight",
        };

        lines.push(Line::from(vec![
            Span::styled(format!("[{}] {}", entry.id, src), entry_style),
            Span::raw(" "),
            Span::styled(&entry.text, entry_style),
        ]));

        lines.push(Line::from(
            Span::styled("[r]esolve [d]ismiss [a]rchive", Style::default().fg(Color::Blue)),
        ));

        lines.push(Line::from(""));
    }

    if lines.is_empty() {
        lines.push(Line::from("No triage entries"));
    }

    let para = Paragraph::new(lines)
        .block(Block::default().title("Triage").borders(Borders::NONE));

    f.render_widget(para, area);
    render_full_border(f, area, &style, true, false);
}
