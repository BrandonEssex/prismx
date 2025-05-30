use ratatui::{backend::Backend, style::{Modifier, Style}, widgets::Paragraph, Frame};
use crate::ui::layout::Rect;
use crate::state::AppState;
use crate::config::theme::ThemeConfig;
use crate::modules::switcher::MODULES;
use crate::plugin::registry;

pub fn render_dock<B: Backend>(f: &mut Frame<B>, area: Rect, state: &mut AppState) {
    if !state.favorite_dock_enabled {
        return;
    }

    // Build entries from built-in modules
    let mut entries: Vec<(String, String)> = MODULES
        .iter()
        .map(|(icon, label)| ((*icon).to_string(), format!("/{}", label.to_lowercase())))
        .collect();

    // Append any plugins discovered via the registry
    entries.extend(registry::dock_entries());

    state.dock_entry_bounds.clear();

    let beam = state.beam_style_for_mode(&state.mode);
    let accent = ThemeConfig::load().accent_color();

    let y = area.y + area.height.saturating_sub(2);
    let mut x = area.x + 1;

    for (i, (icon, command)) in entries.iter().enumerate() {
        let rect = Rect::new(x, y, 2, 1);
        let mut style = Style::default().fg(beam.status_color);
        if state.favorite_focus_index == Some(i) || state.dock_hover_index == Some(i) {
            style = Style::default().fg(accent).add_modifier(Modifier::REVERSED);
        }
        f.render_widget(Paragraph::new(icon.as_str()).style(style), rect);
        state.dock_entry_bounds.push((rect, command.clone()));
        x += 3;
    }

    if state.dock_pulse_frames > 0 && std::env::var("PRISMX_TEST").is_err() {
        state.dock_pulse_frames -= 1;
    }
}
