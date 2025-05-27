use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Clear, Wrap};
use ratatui::text::{Line, Span};

use crate::state::AppState;
use crate::plugin::registry::load_registry;
use crate::plugin::registry::registry_filtered;
use crate::state::PluginTagFilter;

use chrono::Datelike;


pub fn render_plugin_panel<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    let style = state.beam_style_for_mode("settings");
    let entries = registry_filtered(state.plugin_tag_filter);
    let filter_label = match state.plugin_tag_filter {
        PluginTagFilter::All => "All",
        PluginTagFilter::Trusted => "#trusted",
        PluginTagFilter::Debug => "#debug",
    };

    if entries.is_empty() {
        let para = Paragraph::new("No plugins available")
            .block(Block::default().title("Plugins").borders(Borders::NONE));
        f.render_widget(para, area);
        crate::beamx::render_full_border(f, area, &style, true, false);
        return;
    }

    let mut constraints = Vec::new();
    constraints.push(Constraint::Length(1));
    for _ in &entries {
        constraints.push(Constraint::Length(5));
    }
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);

    let header = chunks[0];
    let cards = &chunks[1..];

    let header_para = Paragraph::new(format!("Filter: {}", filter_label))
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(header_para, header);

    for (i, (rect, entry)) in cards.iter().zip(entries.iter()).enumerate() {
        let mut block_style = Style::default().fg(if entry.trusted { Color::Green } else { Color::Red });
        if i == state.plugin_registry_index {
            block_style = block_style.add_modifier(Modifier::BOLD);
        }
        let title = format!("{} v{}", entry.name, entry.version);
        let block = Block::default().borders(Borders::ALL).title(title).style(block_style);

        let tag_line = entry
            .tags
            .iter()
            .map(|t| format!("[#{}]", t))
            .collect::<Vec<_>>()
            .join(" ");

        let lines = vec![
            Line::from(Span::raw(entry.description)),
            Line::from(Span::styled(tag_line, Style::default().fg(Color::Blue))),
        ];
        let para = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });
        f.render_widget(para, *rect);
    }

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
