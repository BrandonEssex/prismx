use ratatui::text::{Span};
use ratatui::style::{Style, Color};
use crate::state::ExportSummary;

pub fn render_export_overlay(export: &ExportSummary) -> Vec {
vec![
Span::styled(
format!(“Exported {} nodes”, export.node_count),
Style::default().fg(Color::Green),
),
Span::styled(
format!(“Exported at: {}”, export.export_time),
Style::default().fg(Color::Cyan),
),
]
}