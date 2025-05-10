use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::layout::Rect;
use ratatui::text::Text;
use ratatui::style::{Style, Color};
use ratatui::Frame;

use crate::plugin::status::PluginStatus;

pub fn render_plugin_dashboard(f: &mut Frame, area: Rect, statuses: &[PluginStatus]) {
    let lines: Vec<String> = statuses.iter().map(|status| format!("{:?}", status)).collect();
    let content = Text::from(lines.join("\n"));

    let paragraph = Paragraph::new(content)
        .block(Block::default().title("Plugin Status").borders(Borders::ALL))
        .style(Style::default().fg(Color::Cyan));

    f.render_widget(paragraph, area);
}