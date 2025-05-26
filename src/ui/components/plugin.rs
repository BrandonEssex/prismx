use ratatui::{backend::Backend, layout::Rect, Frame};

use crate::state::AppState;
use crate::plugin::panel::render_plugin_panel;
use crate::plugin::registry;
use ratatui::widgets::Paragraph;
use ratatui::style::{Style, Color};

pub fn render_plugin<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    registry::tick();
    render_plugin_panel(f, area, state);
    if registry::sync_badge() {
        let label = Paragraph::new("[sync]").style(Style::default().fg(Color::Green));
        let width = 6u16;
        f.render_widget(label, Rect::new(area.right().saturating_sub(width + 1), area.y, width, 1));
    }
}
