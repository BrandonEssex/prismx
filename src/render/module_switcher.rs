use ratatui::{backend::Backend, layout::Rect, style::Style, widgets::{Block, Borders, Paragraph, Wrap}, Frame};

pub fn render_module_switcher<B: Backend>(f: &mut Frame<B>, area: Rect, index: usize) {
    let modules = ["Mindmap", "Zen", "Settings", "Triage"];
    let selected = modules[index % modules.len()];
    let width = 30;
    let height = 5;

    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;

    let block = Block::default()
        .title("Switch Module")
        .borders(Borders::ALL)
        .style(Style::default().fg(ratatui::style::Color::Magenta));

    let content = Paragraph::new(format!("[ {} ]", selected))
        .block(block)
        .wrap(Wrap { trim: false });

    f.render_widget(content, Rect::new(x, y, width, height));
}
