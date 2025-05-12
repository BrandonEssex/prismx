use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::plugin::registry::PluginRegistry;

pub fn render_plugin_dashboard<B: Backend>(f: &mut Frame<B>, area: Rect, registry: &PluginRegistry) {
    let slots = registry.all();

    let lines: Vec<Spans> = slots
        .iter()
        .map(|slot| {
            Spans::from(vec![
                Span::styled(
                    format!("{} ", slot.id),
                    Style::default().fg(Color::Green),
                ),
                Span::raw(format!(
                    "- {} ({})",
                    slot.display_name,
                    if slot.active { "Active" } else { "Inactive" }
                )),
            ])
        })
        .collect();

    let paragraph = Paragraph::new(lines)
        .block(Block::default().title("Plugin Dashboard").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));

    f.render_widget(paragraph, area);
}