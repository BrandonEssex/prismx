use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::state::AppState;
use crate::config_store::theme::ThemeConfig;
use super::{layout::settings_area, toggle::SETTING_TOGGLES};

pub fn render_settings<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    let theme = ThemeConfig::load();
    let mut lines: Vec<Line> = Vec::new();

    for (i, t) in SETTING_TOGGLES.iter().enumerate() {
        let enabled = (t.is_enabled)(state);
        let selected = i == state.settings_focus_index % SETTING_TOGGLES.len();
        let label = t.label;

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

        if i == 2 || i == 3 {
            lines.push(Line::default());
        }
    }

    let rect = settings_area(area, &lines);

    let block = Block::default().title("Settings").borders(Borders::ALL);
    let content = Paragraph::new(lines).block(block).wrap(Wrap { trim: false });

    f.render_widget(content, rect);
}
