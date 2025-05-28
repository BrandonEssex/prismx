use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::render::traits::Renderable;
use crate::state::AppState;

pub fn render_module_switcher<B: Backend>(f: &mut Frame<B>, area: Rect, index: usize) {
    let modules = [
        ("💭", "Mindmap"),
        ("🧘", "Zen"),
        ("🧭", "Triage"),
        ("⚙️", "Settings"),
        ("🔌", "Plugins"),
    ];

    let lines: Vec<Line> = modules
        .iter()
        .enumerate()
        .map(|(i, (icon, label))| {
            let selected = i == index % modules.len();
            let prefix = if selected { "> " } else { "  " };
            let style = if selected {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            };
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

    let width = content_width.min(area.width);
    let mut height = lines.len() as u16 + 2;
    // Keep overlay above the status bar
    height = height.min(area.height.saturating_sub(1));

    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;

    let block = Block::default().title("Switch Module").borders(Borders::ALL);

    let content = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false });

    f.render_widget(content, Rect::new(x, y, width, height));
}

/// Renderable wrapper for the module switcher overlay.
pub struct ModuleSwitcher<'a> {
    pub state: &'a AppState,
}

impl<'a> ModuleSwitcher<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }
}

impl<'a> Renderable for ModuleSwitcher<'a> {
    fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        render_module_switcher(f, area, self.state.module_switcher_index);
    }
}
