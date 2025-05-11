use ratatui::{layout::Rect, Frame};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::text::{Line, Span};

use crate::plugin::registry::PluginRegistry;

pub fn register_sample() {
    PluginRegistry::register_slot("plugin:sample", draw_sample);
}

fn draw_sample(f: &mut Frame<'_>, area: Rect) {
    let content = Line::from(vec![
        Span::raw("Plugin: Sample Running"),
    ]);

    let block = Paragraph::new(content)
        .block(Block::default().title("Sample").borders(Borders::ALL));

    f.render_widget(block, area);
}