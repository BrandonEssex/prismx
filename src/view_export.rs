use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::layout::Rect;
use ratatui::text::{Span, Line, Text};
use ratatui::style::{Style, Color};
use ratatui::Frame;

use crate::state::ExportSummary;

pub fn render_export_panel(f: &mut Frame, area: Rect, export: &ExportSummary) {
    let lines = vec![
        Line::from(Span::styled(
            format!("Exported {} nodes", export.node_count),
            Style::default().fg(Color::Green),
        )),
        Line::from(Span::styled(
            format!("Exported at: {}", export.export_time),
            Style::default().fg(Color::Cyan),
        )),
    ];

    let paragraph = Paragraph::new(Text::from(lines))
        .block(Block::default().title("Export Summary").borders(Borders::ALL));

    f.render_widget(paragraph, area);
}