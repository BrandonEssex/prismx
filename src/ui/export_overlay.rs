use crate::state::export_summary::ExportSummary;
use ratatui::{
    widgets::{Block, Borders, Paragraph},
    layout::Rect,
    text::{Span, Spans, Text},
    style::{Style, Color},
    Frame,
};

pub fn render_export_overlay(f: &mut Frame<'_>, area: Rect, export: &ExportSummary) {
    let lines = vec![
        Spans::from(vec![
            Span::styled(format!("Exported {} nodes", export.node_count), Style::default().fg(Color::Cyan)),
        ]),
        Spans::from(vec![
            Span::styled(format!("Exported at: {}", export.export_time), Style::default().fg(Color::Yellow)),
        ]),
    ];

    let block = Paragraph::new(Text::from(lines))
        .block(Block::default().title("Export Summary").borders(Borders::ALL));

    f.render_widget(block, area);
}