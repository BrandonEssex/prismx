use ratatui::{
    layout::{Rect},
    widgets::{Block, Borders, Paragraph, List, ListItem},
    text::{Line, Span},
    style::{Style, Color},
    Frame,
};
use crate::state::ExportSummary;

pub fn render_export_overlay<B: ratatui::backend::Backend>(
    f: &mut Frame<B>,
    area: Rect,
    export: &ExportSummary,
) {
    let block = Block::default().title("Export Summary").borders(Borders::ALL);
    let lines = vec![
        Line::from(vec![
            Span::raw("Format: "),
            Span::styled(&export.format, Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::raw("Tags Included: "),
            Span::styled(export.tags.join(", "), Style::default().fg(Color::Magenta)),
        ]),
        Line::from(vec![
            Span::raw("Trust Level: "),
            Span::styled(&export.trust_summary, Style::default().fg(Color::Yellow)),
        ]),
    ];

    let para = Paragraph::new(lines).block(block);
    f.render_widget(para, area);
}