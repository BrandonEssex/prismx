use ratatui::{backend::Backend, layout::Rect, style::Style, widgets::{Block, Borders, Paragraph}, Frame};
use crate::beamx::{render_beamx, render_full_border, style_for_mode, BeamXStyle};

pub fn render_triage<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let style = style_for_mode("triage");
    let block = Block::default().title("Triage Panel").borders(Borders::NONE)
        .style(Style::default().fg(ratatui::style::Color::Red));

    let content = Paragraph::new(
        "• GemX rendering: OK\n\
         • Node editing: OK\n\
         • Zen scroll: OK\n\
         • Triage display: Working"
    );

    f.render_widget(block, area);
    f.render_widget(content, Rect::new(area.x + 2, area.y + 1, area.width - 4, area.height - 2));
    render_full_border(f, area, &style);
    render_beamx(f, area, &style, BeamXStyle::Split);
}
