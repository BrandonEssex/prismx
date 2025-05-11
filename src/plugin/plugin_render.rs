use ratatui::{layout::Rect, Frame};

use crate::plugin::registry::PluginRegistry;

/// Example plugin-facing render interface
pub fn register_sample_plugin() {
    PluginRegistry::register_slot("dashboard:bottom", draw_sample_plugin_panel);
}

fn draw_sample_plugin_panel(f: &mut Frame<'_>, area: Rect) {
    use ratatui::widgets::{Block, Borders, Paragraph};
    use ratatui::text::{Span, Line};

    let content = Line::from(vec![
        Span::raw("Sample Plugin Dashboard View"),
    ]);

    let widget = Paragraph::new(content)
        .block(Block::default().title("SamplePlugin").borders(Borders::ALL));

    f.render_widget(widget, area);
}