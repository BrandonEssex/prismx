use ratatui::{backend::Backend, style::{Color, Modifier, Style}, widgets::Paragraph, Frame};
use crate::ui::layout::Rect;
use crate::ui::borders::draw_rounded_border;
use crate::state::{AppState, DockLayout};
use crate::config::theme::ThemeConfig;
use crate::theme::characters::{HORIZONTAL, VERTICAL};

pub fn render_dock<B: Backend>(f: &mut Frame<B>, area: Rect, state: &mut AppState) {
    if !state.favorite_dock_enabled {
        return;
    }

    let favorites = state.favorite_entries();
    state.dock_entry_bounds.clear();

    let beam = state.beam_style_for_mode(&state.mode);
    let accent = ThemeConfig::load().accent_color();

    let y = area.y + area.height.saturating_sub(2);
    let mut x = area.x + 1;

    for (i, entry) in favorites.iter().enumerate() {
        let rect = Rect::new(x, y, 2, 1);
        let mut style = Style::default().fg(beam.status_color);
        if state.favorite_focus_index == Some(i) || state.dock_hover_index == Some(i) {
            style = Style::default().fg(accent).add_modifier(Modifier::REVERSED);
        }
        f.render_widget(Paragraph::new(entry.icon).style(style), rect);
        state.dock_entry_bounds.push((rect, entry.command.to_string()));
        x += 3;
    }

    if state.dock_pulse_frames > 0 && std::env::var("PRISMX_TEST").is_err() {
        state.dock_pulse_frames -= 1;
    }
}
