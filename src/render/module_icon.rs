use ratatui::{backend::Backend, layout::Rect, style::{Color, Style, Modifier}, widgets::Paragraph, Frame};

pub fn module_icon(mode: &str) -> &'static str {
    match mode {
        "gemx" => "💭",
        "zen" => "✍️",
        "triage" => "🧭",
        "spotlight" => "🔍",
        _ => "❓",
    }
}

pub fn render_module_icon<B: Backend>(f: &mut Frame<B>, area: Rect, mode: &str) {
    let glyph = module_icon(mode);
    let style = Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD);
    let x = area.x + 1;
    let y = area.y;
    f.render_widget(Paragraph::new(glyph).style(style), Rect::new(x, y, 2, 1));
}
