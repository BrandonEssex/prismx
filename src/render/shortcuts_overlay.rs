use ratatui::{
    backend::Backend,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::ui::shortcuts::action_group;
use crate::plugin::hook::get_hotkeys;
use crate::ui::animate;
use crate::state::AppState;
use ratatui::{style::{Style, Modifier}, text::{Line, Span}, widgets::Clear};

pub fn render_shortcuts_overlay<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    use std::collections::BTreeMap;

    let mut groups: BTreeMap<&str, Vec<(String, String)>> = BTreeMap::new();
    for (act, key) in &state.hotkeys {
        let g = action_group(act);
        groups.entry(g).or_default().push((key.clone(), act.replace('_', " ")));
    }

    for (k, v) in get_hotkeys() {
        groups.entry("Plugins").or_default().push((k, v));
    }

    let active_group = match state.mode.as_str() {
        "zen" => "Zen",
        "triage" => "Triage",
        "plugin" => "Plugins",
        "settings" => "Settings",
        _ => "GemX",
    };

    let mut lines: Vec<Line> = Vec::new();

    if let Some(mut items) = groups.remove(active_group) {
        lines.push(Line::from(Span::styled(
            active_group,
            Style::default().add_modifier(Modifier::BOLD),
        )));
        for (k, v) in items.drain(..) {
            lines.push(Line::from(format!("{} = {}", k, v)));
        }
        lines.push(Line::from(""));
    }

    if let Some(mut items) = groups.remove("Debug") {
        if state.debug_overlay || state.debug_overlay_sticky {
            lines.push(Line::from(Span::styled(
                "Debug",
                Style::default().add_modifier(Modifier::BOLD),
            )));
            for (k, v) in items.drain(..) {
                lines.push(Line::from(format!("{} = {}", k, v)));
            }
            lines.push(Line::from(""));
        }
    }

    if let Some(mut items) = groups.remove("Global") {
        lines.push(Line::from(Span::styled(
            "Global",
            Style::default().add_modifier(Modifier::BOLD),
        )));
        for (k, v) in items.drain(..) {
            lines.push(Line::from(format!("{} = {}", k, v)));
        }
        lines.push(Line::from(""));
    }

    let content_width = lines.iter().map(|l| l.width() as u16).max().unwrap_or(0).saturating_add(4);
    let mut height = lines.len() as u16 + 2;
    height = height.min(area.height.saturating_sub(1));
    let base_width = content_width.min(area.width);

    let scale = animate::soft_bounce(state.keymap_animation_frame, state.keymap_closing);
    let width = ((base_width as f32) * scale) as u16;
    let width = width.max(3).min(area.width);

    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;

    let title = format!("{} Shortcuts", crate::theme::icons::terminal_icon());
    let block = Block::default().title(title).borders(Borders::ALL);
    let content = Paragraph::new(lines).block(block);

    let rect = Rect::new(x, y, width, height);
    f.render_widget(Clear, rect);
    f.render_widget(content, rect);
}
