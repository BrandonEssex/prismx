use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    text::{Line},
};
use crate::theme::get_style;
use crate::gemx::nodes::MindmapNode;

pub fn render_mindmap<B: ratatui::backend::Backend>(f: &mut Frame<B>, area: Rect, node: &MindmapNode) {
    let lines = vec![
        Line::from(format!("🧠 {}", node.title)),
    ];

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

    let paragraph = Paragraph::new(vec![Line::from(format!("> {}", input))])
        .block(block);
    f.render_widget(paragraph, area);
}

pub fn render_keymap_overlay<B: ratatui::backend::Backend>(f: &mut Frame<B>, area: Rect) {
    let shortcuts = vec![
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

    let paragraph = Paragraph::new(shortcuts).block(block);
    f.render_widget(paragraph, area);
}

pub fn render_clipboard<B: ratatui::backend::Backend>(f: &mut Frame<B>, area: Rect, last_copied: &str) {
    let block = Block::default()
        .title("Clipboard")
        .borders(Borders::ALL)
        .style(get_style("clipboard"));

    let paragraph = Paragraph::new(vec![Line::from(last_copied)])
        .block(block);
    f.render_widget(paragraph, area);
}
