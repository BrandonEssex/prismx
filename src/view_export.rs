use crate::state::export_summary::ExportSummary;
use ratatui::text::{Spans, Span, Text};

pub fn render_export_text(export: &ExportSummary) -> Text<'_> {
    Text::from(vec![
        Spans::from(vec![
            Span::raw(format!("Exported {} nodes", export.node_count))
        ]),
        Spans::from(vec![
            Span::raw(format!("Exported at: {}", export.export_time))
        ])
    ])
}