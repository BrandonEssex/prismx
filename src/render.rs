pub fn render_dashboard<B: ratatui::backend::Backend>(f: &mut Frame<B>, area: Rect) {
    let lines = vec![
        Line::from("Trust Score: 100"),
        Line::from("Plugins: gemx, dashboard, mindtrace"),
        Line::from("Federation Drift: 0"),
    ];

    let block = Block::default()
        .title("Dashboard")
        .borders(Borders::ALL)
        .style(get_style("dashboard"));

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);
}

pub fn render_zen_journal<B: ratatui::backend::Backend>(f: &mut Frame<B>, area: Rect) {
    let block = Block::default()
        .title("Zen Journal")
        .borders(Borders::ALL)
        .style(get_style("zen"));

    let paragraph = Paragraph::new(vec![Line::from("Write here... (Ctrl+D to export)")])
        .block(block);
    f.render_widget(paragraph, area);
}
