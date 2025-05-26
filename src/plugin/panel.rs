use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Clear, Wrap};
use ratatui::text::{Line, Span};

use crate::state::AppState;
use crate::plugin::registry::{registry, PluginEntry};

pub fn render_plugin_panel<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    let style = state.beam_style_for_mode("settings");
    let entries = registry();

    let mut lines: Vec<Line> = Vec::new();
    for (i, PluginEntry { name, description, .. }) in entries.iter().enumerate() {
        let selected = i == state.plugin_registry_index;
        let prefix = if selected { "> " } else { "  " };
        let mut style_line = Style::default();
        if selected {
            style_line = style_line.fg(Color::Cyan).add_modifier(Modifier::BOLD);
        }
        lines.push(Line::from(vec![
            Span::styled(prefix.to_string(), style_line),
            Span::styled(*name, style_line.add_modifier(Modifier::BOLD)),
        ]));
        lines.push(Line::from(Span::styled(*description, Style::default().fg(Color::Gray))));
        lines.push(Line::from(Span::styled("[install]", Style::default().fg(Color::DarkGray))));
        lines.push(Line::from(""));
    }

    if lines.is_empty() {
        lines.push(Line::from("No plugins available"));
    }

    let para = Paragraph::new(lines)
        .block(Block::default().title("Plugins").borders(Borders::NONE));

    f.render_widget(para, area);
    crate::beamx::render_full_border(f, area, &style, true, false);

    if state.show_plugin_preview {
        if let Some(entry) = entries.get(state.plugin_registry_index) {
            let mut details = vec![
                Line::from(Span::styled(
                    entry.name,
                    Style::default().add_modifier(Modifier::BOLD),
                )),
                Line::from(format!("Version: {}", entry.version)),
                Line::from(entry.description),
                Line::from(format!(
                    "Trusted: {}",
                    if entry.trusted { "yes" } else { "no" }
                )),
                Line::from(format!("Trust chain: {}", entry.trust_chain)),
                Line::from(Span::styled("[Install]", Style::default().fg(Color::DarkGray))),
            ];

            let content_width = details
                .iter()
                .map(|l| l.width() as u16)
                .max()
                .unwrap_or(0)
                .saturating_add(4);
            let width = content_width.min(area.width);
            let mut height = details.len() as u16 + 2;
            height = height.min(area.height.saturating_sub(1));

            let x = area.x + (area.width.saturating_sub(width)) / 2;
            let y = area.y + (area.height.saturating_sub(height)) / 2;

            let block = Block::default().title("Plugin Preview").borders(Borders::ALL);
            let rect = Rect::new(x, y, width, height);
            f.render_widget(Clear, rect);
            let para = Paragraph::new(details).block(block).wrap(Wrap { trim: false });
            f.render_widget(para, rect);
        }
    }
}
