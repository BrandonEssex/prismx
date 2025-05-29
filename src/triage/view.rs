use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::layout::{Layout, Constraint, Direction};
use ratatui::text::{Line, Span};

use crate::state::AppState;
use crate::triage::state::{TriageSource, tag_counts};
use crate::ui::components::status::render_triage_status;
use crate::beamx::render_full_border;


#[allow(dead_code)]
fn draw_plain_border<B: Backend>(f: &mut Frame<B>, area: Rect, color: Color) {
    let style = Style::default().fg(color);
    let right = area.x + area.width.saturating_sub(1);
    let bottom = area.y + area.height.saturating_sub(1);

    for x in area.x + 1..right {
        f.render_widget(Paragraph::new("─").style(style), Rect::new(x, area.y, 1, 1));
        f.render_widget(Paragraph::new("─").style(style), Rect::new(x, bottom, 1, 1));
    }
    for y in area.y + 1..bottom {
        f.render_widget(Paragraph::new("│").style(style), Rect::new(area.x, y, 1, 1));
        f.render_widget(Paragraph::new("│").style(style), Rect::new(right, y, 1, 1));
    }
}

pub fn render_triage_panel<B: Backend>(f: &mut Frame<B>, area: Rect, state: &mut AppState) {
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
    let (now_count, triton_count, done_count) = tag_counts(state);
    {
        let metrics = &mut state.triage_summary;
        if (now_count, triton_count, done_count)
            != (metrics.now, metrics.triton, metrics.done)
        {
            if metrics.triton > triton_count && done_count > metrics.done {
                metrics.last_action = Some("#TRITON → #DONE".into());
            } else if metrics.now > now_count && triton_count > metrics.triton {
                metrics.last_action = Some("#NOW → #TRITON".into());
            }
            metrics.now = now_count;
            metrics.triton = triton_count;
            metrics.done = done_count;
            metrics.highlight_frames = 4;
        }
    }
    render_triage_status(f, zones[0], state);

    // --- Left Feed ---
    let mut lines = Vec::new();

    // Render active tag filters (#TRITON, #DONE, etc.) if present
    use std::collections::BTreeSet;
    let mut active_tags: BTreeSet<String> = BTreeSet::new();
    let mut visible_idx = 0usize;
    for entry in &state.triage_entries {
        if entry.archived { continue; }
        for tag in &entry.tags {
            active_tags.insert(tag.to_uppercase());
        }
    }
    if !active_tags.is_empty() {
        let tags = active_tags.iter().cloned().collect::<Vec<_>>().join(" ");
        lines.push(Line::from(Span::styled(
            format!("Filters: {}", tags),
            Style::default().fg(Color::Yellow),
        )));
        lines.push(Line::from(""));
    }

    for entry in &state.triage_entries {
        if entry.archived { continue; }

        let mut entry_style = Style::default();
        if visible_idx == state.triage_focus_index {
            entry_style = entry_style.add_modifier(Modifier::BOLD);
        }
        if entry.resolved {
            entry_style = entry_style
                .fg(Color::DarkGray)
                .add_modifier(Modifier::DIM);
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

        if !entry.resolved {
            lines.push(Line::from(
                Span::styled(
                    "[r]esolve [d]ismiss [a]rchive",
                    Style::default().fg(Color::Blue),
                ),
            ));
        }

        lines.push(Line::from(""));
        visible_idx += 1;
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
