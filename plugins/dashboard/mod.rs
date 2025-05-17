use ratatui::{
    widgets::{Block, Borders, Paragraph},
    text::{Span, Spans},
};

pub fn render_panel() -> Paragraph<'static> {
    let title = "DASHBOARD";
    let trust_status = "Trust Score: 100";
    let loaded_plugins = "Plugins: gemx, dashboard, mindtrace";
    let drift = "Federation Drift: 0";

    Paragraph::new(vec![
        Spans::from(vec![Span::raw(trust_status)]),
        Spans::from(vec![Span::raw(loaded_plugins)]),
        Spans::from(vec![Span::raw(drift)]),
    ])
    .block(Block::default().title(title).borders(Borders::ALL))
}
