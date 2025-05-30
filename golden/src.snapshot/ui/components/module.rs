use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap, Clear},
    Frame,
};

use crate::state::AppState;
use crate::ui::animate;
use crate::config::theme::ThemeConfig;

pub fn render_module_switcher<B: Backend>(
    f: &mut Frame<B>,
    area: Rect,
    state: &AppState,
) {
    let modules = [
        ("💭", "Mindmap"),
        ("🧘", "Zen"),
        ("🧭", "Triage"),
        ("⚙️", "Settings"),
        ("🔌", "Plugins"),
    ];

    let count = modules.len();
    let index = state.module_switcher_index % count;

    // Determine visible range if many modules
    let max_visible = 5usize;
    let start = if count > max_visible {
        let half = max_visible / 2;
        if index < half {
            0
        } else if index + half >= count {
            count - max_visible
        } else {
            index - half
        }
    } else {
        0
    };

    let accent = ThemeConfig::load().accent_color();

    let lines: Vec<Line> = modules
        .iter()
        .enumerate()
        .skip(start)
        .take(max_visible.min(count))
        .map(|(i, (icon, label))| {
            let selected = i == index;
            let prefix = if selected { "> " } else { "  " };
            let mut style = if selected {
                animate::shimmer(accent, state.module_switcher_animation_frame as u64)
            } else {
                Style::default().fg(Color::Gray)
            };
            if selected {
                style = style.add_modifier(Modifier::BOLD);
            }
            Line::from(vec![
                Span::styled(prefix.to_string(), style),
                Span::styled(format!("{} {}", icon, label), style),
            ])
        })
        .collect();

    let content_width = lines
        .iter()
        .map(|l| l.width() as u16)
        .max()
        .unwrap_or(0)
        .saturating_add(4);

    let base_width = content_width.min(area.width);
    let mut height = lines.len() as u16 + 2;
    height = height.min(area.height.saturating_sub(1));

    let scale = animate::soft_bounce(state.module_switcher_animation_frame, state.module_switcher_closing);
    let width = ((base_width as f32) * scale) as u16;
    let width = width.max(3).min(area.width);

    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;

    let block = Block::default()
        .title("Switch Module")
        .borders(Borders::ALL)
        .style(Style::default().fg(accent));

    let content = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false });

    let bg = Rect::new(x, y, width, height);
    f.render_widget(Clear, bg);
    f.render_widget(content, bg);
}
