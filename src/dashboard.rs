use ratatui::{
    widgets::{Block, Borders, Paragraph},
    text::{Line},
};

pub fn render_panel() -> Paragraph<'static> {
    let title = "DASHBOARD";

    let trust_status = "Trust Score: 100";
    let plugin_status = "Plugins: gemx, dashboard, mindtrace";
    let drift_status = "Federation Drift: 0";

    let lines = vec![
        Line::from(trust_status),
        Line::from(plugin_status),
        Line::from(drift_status),
    ];

    Paragraph::new(lines)
        .block(Block::default().title(title).borders(Borders::ALL))
}
