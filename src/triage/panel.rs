use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::layout::{Layout, Constraint, Direction};
use ratatui::text::{Line, Span};

use crate::state::AppState;
use crate::triage::logic::TriageSource;
use crate::beamx::render_full_border;

pub fn render_triage_panel<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    let style = state.beam_style_for_mode("triage");
    let block_style = Style::default().fg(style.border_color);

    // Divide panel into summary (top) and two side zones.
    let zones = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(0)].as_ref())
        .split(area);

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(zones[1]);

    // --- Summary Bar ---
    let open_count = state
        .triage_entries
        .iter()
        .filter(|e| !e.archived && !e.resolved)
        .count();
    let summary = Paragraph::new(format!("{open_count} open entries"))
        .block(Block::default().borders(Borders::NONE).style(block_style));
    f.render_widget(summary, zones[0]);

    // --- Left Feed ---
    let mut lines = Vec::new();
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

    let feed = Paragraph::new(lines)
        .block(Block::default().borders(Borders::NONE).style(block_style));
    f.render_widget(feed, body[0]);

    // --- Right Stats ---
    use std::collections::BTreeMap;
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for entry in &state.triage_entries {
        if entry.archived { continue; }
        for tag in &entry.tags {
            *counts.entry(tag.clone()).or_insert(0) += 1;
        }
    }
    let mut stats_lines = Vec::new();
    if counts.is_empty() {
        stats_lines.push(Line::from(""));
    } else {
        for (tag, count) in counts {
            stats_lines.push(Line::from(format!("{}: {}", tag, count)));
        }
    }
    let stats = Paragraph::new(stats_lines)
        .block(Block::default().borders(Borders::NONE).style(block_style));
    f.render_widget(stats, body[1]);

    render_full_border(f, area, &style, true, false);
}
