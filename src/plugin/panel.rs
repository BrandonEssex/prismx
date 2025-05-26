use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::text::{Line, Span};

use crate::state::AppState;
use crate::plugin::registry::{registry, PluginEntry};

pub fn render_plugin_panel<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    let style = state.beam_style_for_mode("settings");
    let entries = registry();

    let mut lines: Vec<Line> = Vec::new();
    for PluginEntry { name, description } in entries {
        lines.push(Line::from(Span::styled(name, Style::default().add_modifier(Modifier::BOLD))));
        lines.push(Line::from(description));
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
}
