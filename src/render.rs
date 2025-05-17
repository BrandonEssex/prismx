use ratatui::{
    Frame,
    prelude::Rect,
    widgets::{Block, Borders, Paragraph},
    text::Line,
};
use crate::theme::get_style;
use crate::gemx::nodes::MindmapNode;

pub fn render_mindmap<B: ratatui::backend::Backend>(f: &mut Frame<B>, area: Rect, node: &MindmapNode) {
    let mut lines = vec![Line::from(format!("🧠 {}", node.title))];
    for (i, child) in node.children.iter().enumerate() {
        lines.push(Line::from(format!("  {}. {}", i + 1, child.title)));
    }

    let block = Block::default()
        .title("GemX Mindmap")
        .borders(Borders::ALL)
        .style(get_style("mindmap"));

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);
}

pub fn render_spotlight<B: ratatui::backend::Backend>(f: &mut Frame<B>, area: Rect, input: &str) {
    let block = Block::default()
        .title("Spotlight")
        .borders(Borders::ALL)
        .style(get_style("spotlight"));

    let paragraph = Paragraph::new(vec![Line::from(format!("> {}", input))]).block(block);
    f.render_widget(paragraph, area);
}

pub fn render_keymap_overlay<B: ratatui::backend::Backend>(f: &mut Frame<B>, area: Rect) {
    let lines = vec![
        Line::from("Ctrl+Q → Quit"),
        Line::from("Ctrl+Z → Zen"),
        Line::from("Ctrl+D → Dashboard"),
        Line::from("Ctrl+O → Spotlight"),
        Line::from("Ctrl+T → Theme Toggle"),
    ];

    let block = Block::default()
        .title("Keymap")
        .borders(Borders::ALL)
        .style(get_style("keymap"));

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);
}

pub fn render_clipboard<B: ratatui::backend::Backend>(f: &mut Frame<B>, area: Rect, last_copied: &str) {
    let block = Block::default()
        .title("Clipboard")
        .borders(Borders::ALL)
        .style(get_style("clipboard"));

    let paragraph = Paragraph::new(vec![Line::from(last_copied)]).block(block);
    f.render_widget(paragraph, area);
}

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

    let paragraph = Paragraph::new(vec![
        Line::from("Type your journal..."),
        Line::from("Press Ctrl+D to save and exit."),
    ])
    .block(block);

    f.render_widget(paragraph, area);
}
