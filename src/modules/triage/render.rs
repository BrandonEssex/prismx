use chrono::{Local, Duration};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::layout::{Layout, Constraint, Direction};
use ratatui::text::{Line, Span};
use std::collections::{BTreeMap, HashSet};

use crate::beamx::render_full_border;
use crate::state::AppState;
use crate::triage::state::{TriageEntry, TriageSource};

/// Calculate consecutive days with at least one `#DONE` entry.
pub fn completion_streak(entries: &[TriageEntry]) -> usize {
    let days: HashSet<_> = entries
        .iter()
        .filter(|e| e.tags.iter().any(|t| t == "#done"))
        .map(|e| e.created.date_naive())
        .collect();
    let mut streak = 0usize;
    let mut day = Local::now().date_naive();
    loop {
        if days.contains(&day) {
            streak += 1;
            day -= Duration::days(1);
        } else {
            break;
        }
    }
    streak
}

/// Return counts of `#DONE` entries for the last `n` days.
fn done_counts_last_n_days(entries: &[TriageEntry], n: usize) -> Vec<usize> {
    let today = Local::now().date_naive();
    (0..n)
        .map(|i| {
            let day = today - Duration::days((n - 1 - i) as i64);
            entries
                .iter()
                .filter(|e| {
                    e.tags.iter().any(|t| t == "#done") && e.created.date_naive() == day
                })
                .count()
        })
        .collect()
}

/// Render a small sparkline using Unicode blocks for the provided counts.
fn sparkline_from_counts(counts: &[usize]) -> String {
    const BLOCKS: &[char] = &['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
    let max = counts.iter().copied().max().unwrap_or(0);
    counts
        .iter()
        .map(|&c| {
            let idx = if max == 0 {
                0
            } else {
                (c * (BLOCKS.len() - 1) + max - 1) / max
            };
            BLOCKS[idx]
        })
        .collect()
}

/// Generate a sparkline string for the last `n` days of completions.
pub fn done_sparkline(entries: &[TriageEntry], n: usize) -> String {
    let counts = done_counts_last_n_days(entries, n);
    sparkline_from_counts(&counts)
}

/// Progress bar representing ratio of completed tasks.
pub fn progress_bar(now: usize, triton: usize, done: usize) -> String {
    let total = now + triton + done;
    let len = 10usize;
    if total == 0 {
        return format!("[{}]", " ".repeat(len));
    }
    let filled = ((done as f32 / total as f32) * len as f32).round() as usize;
    let filled = filled.min(len);
    format!("[{}{}]", "█".repeat(filled), "░".repeat(len - filled))
}

/// Render triage entries grouped by primary tags (#now, #triton, #done).
/// When `show_icons` is true, tag headers include emoji icons.
pub fn render_grouped<B: Backend>(
    f: &mut Frame<B>,
    area: Rect,
    state: &mut AppState,
    show_icons: bool,
) {
    let style = state.beam_style_for_mode("triage");
    let block_style = Style::default().fg(style.border_color);

    let zones = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0)].as_ref())
        .split(area);

    let body = zones[0];

    let mut groups: BTreeMap<&str, Vec<(usize, &TriageEntry)>> = BTreeMap::new();
    for (idx, entry) in state.triage_entries.iter().enumerate() {
        if entry.archived {
            continue;
        }
        let tag = if entry.tags.iter().any(|t| t == "#now") {
            "#now"
        } else if entry.tags.iter().any(|t| t == "#triton") {
            "#triton"
        } else if entry.tags.iter().any(|t| t == "#done") {
            "#done"
        } else {
            "other"
        };
        groups.entry(tag).or_default().push((idx, entry));
    }

    let order = ["#now", "#triton", "#done", "other"];
    let mut lines: Vec<Line> = Vec::new();
    let mut visible_idx = 0usize;

    for tag in &order {
        if let Some(entries) = groups.get(*tag) {
            if entries.is_empty() {
                continue;
            }
            let header = match *tag {
                "#now" => if show_icons { "🔥 NOW" } else { "#NOW" },
                "#triton" => if show_icons { "🧠 TRITON" } else { "#TRITON" },
                "#done" => if show_icons { "✅ DONE" } else { "#DONE" },
                _ => "OTHER",
            };
            lines.push(Line::from(Span::styled(
                header,
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(""));

            for (idx, entry) in entries {
                let mut entry_style = Style::default();
                if *idx == state.triage_focus_index {
                    entry_style = entry_style
                        .add_modifier(Modifier::BOLD)
                        .bg(Color::DarkGray);
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
        }
    }

    if lines.is_empty() {
        lines.push(Line::from("No triage entries"));
    }

    let feed = Paragraph::new(lines)
        .block(Block::default().borders(Borders::NONE).style(block_style));
    f.render_widget(feed, body);

    if state.sticky_overlay_visible {
        for note in &state.sticky_notes_data {
            note.render(f);
        }
    }

    render_full_border(f, area, &style, true, false);
}
