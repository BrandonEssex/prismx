use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::layout::Rect;
use ratatui::text::Text;
use ratatui::style::{Style, Color};
use ratatui::Frame;

pub fn render_dashboard<B>(f: &mut Frame, area: Rect) {
    let content = Text::from("Dashboard content coming soon.");
    let widget = Paragraph::new(content)
        .block(Block::default().title("Dashboard").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));
    f.render_widget(widget, area);
}