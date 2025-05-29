use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::layout::{Layout, Constraint, Direction};
use ratatui::text::{Line, Span};

use crate::beamx::render_full_border;
use crate::state::AppState;
use crate::triage::state::{TriageEntry, TriageSource};

use std::collections::BTreeMap;

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

    // Split area into just a body for now (no summary bar here).
    let zones = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0)].as_ref())
        .split(area);

    let body = zones[0];

    // Group visible entries by tag
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
        }
    }

    if lines.is_empty() {
        lines.push(Line::from("No triage entries"));
    }

    let feed = Paragraph::new(lines)
        .block(Block::default().borders(Borders::NONE).style(block_style));
    f.render_widget(feed, body);

    render_full_border(f, area, &style, true, false);
}
