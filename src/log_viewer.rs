use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::layout::Rect;
use ratatui::text::Text;
use ratatui::style::{Style, Color};
use ratatui::Frame;

pub fn render_log_viewer(f: &mut Frame, area: Rect, lines: &[String]) {
    let content = Text::from(lines.join("\n"));
    let paragraph = Paragraph::new(content)
        .block(Block::default().title("Log Viewer").borders(Borders::ALL))
        .style(Style::default().fg(Color::Gray));

    f.render_widget(paragraph, area);
}