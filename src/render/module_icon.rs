use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::beamx;

pub fn module_icon(mode: &str) -> &'static str {
    match mode {
        "gemx" => "💭",
        "zen" => "🧘",
        "triage" => "🧭",
        "spotlight" => "🔍",
        "settings" => "⚙️",
        _ => "❓",
    }
}

pub fn render_module_icon<B: Backend>(f: &mut Frame<B>, area: Rect, mode: &str) {
    let glyph = module_icon(mode);
    let label = match mode {
        "gemx" => "GEMX",
        "zen" => "ZEN",
        "triage" => "TRIAGE",
        "spotlight" => "SPOTLIGHT",
        "settings" => "SETTINGS",
        _ => "UNKNOWN",
    };

    let content = format!("{} {}", glyph, label);

    let theme = beamx::style_for_mode(mode);
    let style = Style::default()
        .fg(theme.border_color)
        .add_modifier(Modifier::BOLD);

    let text_width = content.chars().count() as u16;
    let block_width = text_width + 2;
    let height = 3u16;

    let x = area.width.saturating_sub(block_width + 1);
    let y = area.height.saturating_sub(height + 4);

    let border = Block::default().borders(Borders::ALL).style(style);
    f.render_widget(border, Rect::new(x, y, block_width, height));
    f.render_widget(
        Paragraph::new(content).style(style),
        Rect::new(x + 1, y + 1, text_width, 1),
    );
}
