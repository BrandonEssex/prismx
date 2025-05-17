use ratatui::{
    widgets::{Block, Borders, Paragraph},
    text::Line,
};

pub fn render_panel() -> Paragraph<'static> {
    let lines = vec![
        Line::from("Trust Score: 100"),
        Line::from("Plugins: gemx, dashboard, mindtrace"),
        Line::from("Federation Drift: 0"),
    ];

    Paragraph::new(lines)
        .block(Block::default().title("DASHBOARD").borders(Borders::ALL))
}
