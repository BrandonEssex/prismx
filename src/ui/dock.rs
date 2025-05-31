use ratatui::{
    backend::Backend,
    style::{Color, Modifier, Style},
    widgets::{Clear, Paragraph},
    Frame,
};
use crate::ui::layout::Rect;
use crate::state::AppState;
use crate::config::theme::ThemeConfig;
use crate::modules::switcher::MODULES;
use crate::plugin::registry;
use crate::ui::borders::draw_rounded_border;
use crate::render::module_icon::{module_icon, module_label};
use crate::layout::RESERVED_ZONE_W;
use unicode_width::UnicodeWidthStr;
use crate::state::HeartbeatMode;
use crate::ui::beamx::heartbeat_glyph;

use std::time::{SystemTime, UNIX_EPOCH};

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

    let dock_width = entries.len() as u16 * 3;
    let show_heart = matches!(state.mode.as_str(), "zen" | "gemx")
        && state.heartbeat_mode != HeartbeatMode::Silent;
    let heart_space = if show_heart { 3 } else { 0 };

    let zoom_text = format!("Zoom: {:.1}x", state.zoom_scale);
    let zoom_w = zoom_text.len() as u16 + 2;
    let icon_content = format!("{} {}", module_icon(&state.mode), module_label(&state.mode));
    let icon_w = UnicodeWidthStr::width(icon_content.as_str()) as u16 + 2;
    let offset = RESERVED_ZONE_W as u16 + icon_w + zoom_w + 1;

    let y = area.y + area.height.saturating_sub(2);
    let total_width = dock_width + heart_space;
    let mut x = area.right().saturating_sub(total_width + offset);
    if x <= area.x { x = area.x + 1; }

    if show_heart {
        let tick = if std::env::var("PRISMX_TEST").is_ok() { 0 } else {
            (SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis()
                / 600) as u64
        };
        let heart = heartbeat_glyph(tick / 2);
        let heart_style = crate::ui::beamx::heartbeat_style(beam.status_color, tick);
        let rect = Rect::new(x, y, 2, 1);
        f.render_widget(Paragraph::new(heart).style(heart_style), rect);
        x += 3;
    }

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

    render_dock_preview(f, area, state, &entries, true);
}

fn render_dock_preview<B: Backend>(
    f: &mut Frame<B>,
    area: Rect,
    state: &mut AppState,
    entries: &[(String, String)],
    horizontal: bool,
) {
    if let Some(idx) = state.dock_hover_index {
        if let Some((rect, _)) = state.dock_entry_bounds.get(idx) {
            if let Some((icon, cmd)) = entries.get(idx) {
                let text = format!("{} {}", icon, cmd);
                let width = text.len() as u16 + 2;
                let (x, y) = if horizontal {
                    (rect.x.saturating_sub(1), rect.y.saturating_sub(3))
                } else {
                    (rect.right() + 1, rect.y)
                };
                let w = width.min(area.width.saturating_sub(x));
                let h = 3u16;
                let rect = Rect::new(x.min(area.right().saturating_sub(w)), y.max(area.top()), w, h);
                let block_style = Style::default().bg(Color::Black).fg(Color::White);
                f.render_widget(Clear, rect);
                draw_rounded_border(f, rect, block_style);
                f.render_widget(
                    Paragraph::new(text).style(block_style),
                    Rect::new(rect.x + 1, rect.y + 1, rect.width.saturating_sub(2), 1),
                );
            }
        }
    }
}
