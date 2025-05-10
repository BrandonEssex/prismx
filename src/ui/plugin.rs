use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::layout::Rect;
use ratatui::text::Text;
use ratatui::style::{Style, Color};
use ratatui::Frame;

pub fn render_plugin_panel(f: &mut Frame, area: Rect, name: &str, content: &str) {
    let paragraph = Paragraph::new(Text::from(content))
        .block(Block::default().title(name).borders(Borders::ALL))
        .style(Style::default().fg(Color::Magenta));

    f.render_widget(paragraph, area);
}