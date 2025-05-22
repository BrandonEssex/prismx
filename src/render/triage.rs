use ratatui::{backend::Backend, layout::Rect, style::Style, widgets::{Block, Borders, Paragraph}, Frame};

pub fn render_triage<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let block = Block::default().title("Triage Panel").borders(Borders::ALL)
        .style(Style::default().fg(ratatui::style::Color::Red));

    let content = Paragraph::new(
        "• GemX rendering: OK\n\
         • Node editing: OK\n\
         • Zen scroll: OK\n\
         • Triage display: Working"
    );

    f.render_widget(block, area);
    f.render_widget(content, Rect::new(area.x + 2, area.y + 1, area.width - 4, area.height - 2));
}
