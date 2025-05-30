use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap, Clear},
    Frame,
};
use crate::ui::layout::Rect;

use crate::theme::layout::{overlay_width, SWITCHER_ITEM_WIDTH};

use unicode_width::UnicodeWidthStr;

use crate::state::AppState;
use crate::ui::animate;
use crate::config::theme::ThemeConfig;
use crate::render::traits::{Renderable, RenderFrame};
use crate::modules::switcher;

pub fn render_module_switcher(
    f: &mut RenderFrame<'_>,
    area: Rect,
    state: &AppState,
) {
    let modules = switcher::MODULES;

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

    // Use a fixed width shared with Spotlight so the panel does not resize based
    // on its content.
    let base_width = overlay_width(area.width);
    let item_width = SWITCHER_ITEM_WIDTH.min(base_width.saturating_sub(2));

    let lines: Vec<Line> = modules
        .iter()
        .enumerate()
        .skip(start)
        .take(max_visible.min(count))
        .map(|(i, (icon, label))| {
            let selected = i == index;
            let text = format!("{} {}", icon, label);
            let text_w = UnicodeWidthStr::width(text.as_str()) as u16;
            let pad_total = item_width.saturating_sub(text_w);
            let left = pad_total / 2;
            let right = pad_total - left;
            let line_str = format!(
                "{left_pad}{text}{right_pad}",
                left_pad = " ".repeat(left as usize),
                right_pad = " ".repeat(right as usize)
            );

            let style = if selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(accent)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            };
            Line::from(Span::styled(line_str, style))
        })
        .collect();

    // Use a fixed width shared with Spotlight so the panel does not resize based
    // on its content.
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

/// Wrapper implementing [`Renderable`] for the module switcher overlay.
pub struct ModuleSwitcher<'a> {
    pub state: &'a AppState,
}

impl<'a> ModuleSwitcher<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }
}

impl<'a> Renderable for ModuleSwitcher<'a> {
    fn render(&mut self, f: &mut RenderFrame<'_>, area: Rect) {
        render_module_switcher(f, area, self.state);
    }
}
