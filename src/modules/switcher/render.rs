use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, Paragraph},
    text::Span,
};

use crate::config::theme::ThemeConfig;
use crate::modules::switcher::MODULES;
use crate::render::traits::{Renderable, RenderFrame};
use crate::state::AppState;

/// Render the macOS-style switcher overlay.
pub fn render_app_switcher(f: &mut RenderFrame<'_>, area: Rect, state: &AppState) {
    let accent = ThemeConfig::load().accent_color();
    let count = MODULES.len();
    let idx = state.module_switcher_index % count;

    let item_w = 12u16; // width per icon block
    let width = (item_w * count as u16).min(area.width.saturating_sub(2));
    let height = 5u16;
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    let rect = Rect::new(x, y, width, height);

    f.render_widget(Clear, rect);

    for (i, (icon, label)) in MODULES.iter().enumerate() {
        let selected = i == idx;
        let style = if selected {
            Style::default().fg(Color::Black).bg(accent).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Gray)
        };
        let ix = rect.x + i as u16 * item_w;
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(style)
            .title(Span::styled(*label, style));
        let inner = Rect::new(ix, rect.y, item_w, rect.height);
        f.render_widget(block, inner);
        let text_rect = Rect::new(inner.x + (inner.width.saturating_sub(2)) / 2, inner.y + 2, 2, 1);
        f.render_widget(Paragraph::new(*icon).style(style), text_rect);
    }
}

/// Wrapper implementing [`Renderable`] around [`render_app_switcher`].
pub struct AppSwitcher<'a> {
    pub state: &'a AppState,
}

impl<'a> AppSwitcher<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }
}

impl<'a> Renderable for AppSwitcher<'a> {
    fn render(&mut self, f: &mut RenderFrame<'_>, area: Rect) {
        render_app_switcher(f, area, self.state);
    }
}
