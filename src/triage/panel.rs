use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::text::{Line, Span};

use crate::state::AppState;
use crate::triage::logic::{TriageSource, TriageEntry};
use crate::beamx::render_full_border;

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

pub fn render_triage_panel<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    let style = state.beam_style_for_mode("triage");

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

    let para = Paragraph::new(lines)
        .block(Block::default().title("Triage").borders(Borders::NONE));

    f.render_widget(para, area);
    draw_plain_border(f, area, style.border_color);
}
