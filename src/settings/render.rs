use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::state::AppState;
use crate::config::theme::ThemeConfig;
use super::{layout::settings_area, toggle::SETTING_TOGGLES};
use crate::theme::previews::preview_line;

pub fn render_settings<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    let theme = ThemeConfig::load();
    let mut lines: Vec<Line> = Vec::new();
    let header_style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);

    for (i, t) in SETTING_TOGGLES.iter().enumerate() {
        // section headers
        if i == 0 {
            lines.push(Line::from(Span::styled("Font", header_style)));
        } else if i == 1 {
            lines.push(Line::default());
            lines.push(Line::from(Span::styled("UI", header_style)));
        } else if i == 5 {
            lines.push(Line::default());
            lines.push(Line::from(Span::styled("Layout", header_style)));
        }

        let enabled = (t.is_enabled)(state);
        let selected = i == state.settings_focus_index % SETTING_TOGGLES.len();
        let mut label = t.label.to_string();
        if t.label == "Font Style" {
            label = format!("Font Style: {}", state.font_style.name());
        }

        let check = if enabled { "[x]" } else { "[ ]" };
        let prefix = if selected { "> " } else { "  " };

        let mut style = if selected {
            Style::default()
                .fg(theme.focus_outline())
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Gray)
        };
        if selected {
            style = style.bg(Color::Black);
        }

        lines.push(Line::from(vec![
            Span::styled(prefix.to_string(), style),
            Span::styled(format!("{} {} {}", check, t.icon, label), style),
        ]));
    }

    lines.push(Line::default());
    lines.push(preview_line(state.font_style, state.settings_beam_color));

    let rect = settings_area(area, &lines);

    let block = Block::default().title("Settings").borders(Borders::ALL);
    let content = Paragraph::new(lines).block(block).wrap(Wrap { trim: false });

    f.render_widget(content, rect);
}
