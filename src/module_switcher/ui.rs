use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::state::AppState;
use crate::render::traits::{Renderable, RenderFrame};

pub fn render_module_switcher(f: &mut RenderFrame<'_>, area: Rect, _state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    let title = Paragraph::new(Line::from(vec![
        Span::styled("Module Switcher", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
    ])).block(Block::default().borders(Borders::ALL).title("Modules"));

    f.render_widget(title, chunks[0]);
}

/// Overlay for module switching implementing [`Renderable`].
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
