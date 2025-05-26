use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::ui::animate::breath_style;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::beamx;
use unicode_width::UnicodeWidthStr;

pub fn module_icon(mode: &str) -> &'static str {
    match mode {
        "gemx" => "💭",
        "zen" => "🧘",
        "triage" => "🧭",
        "spotlight" => "🔍",
        "settings" => "⚙️",
        "plugin" => "🔌",
        _ => "❓",
    }
}

pub fn module_label(mode: &str) -> &'static str {
    match mode {
        "gemx" => "GemX",
        "zen" => "Zen",
        "triage" => "Triage",
        "spotlight" => "Spotlight",
        "settings" => "Settings",
        "plugin" => "Plugins",
        _ => "Unknown",
    }
}

pub fn render_module_icon<B: Backend>(f: &mut Frame<B>, area: Rect, mode: &str) {
    let glyph = module_icon(mode);
    let label = module_label(mode);

    let content = format!("{} {}", glyph, label);

    let theme = beamx::style_for_mode(mode);
    let tick = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() / 300;
    let style = breath_style(theme.border_color, tick as u64)
        .add_modifier(Modifier::BOLD);

    let text_width = UnicodeWidthStr::width(content.as_str()) as u16;
    let block_width = text_width + 2;
    let height = 3u16;

    let x = area.right().saturating_sub(block_width + 2);
    let y = area.bottom().saturating_sub(height + 4);

    let border = Block::default().borders(Borders::ALL).style(style);
    f.render_widget(border, Rect::new(x, y, block_width, height));
    f.render_widget(
        Paragraph::new(content).style(style),
        Rect::new(x + 1, y + 1, text_width, 1),
    );
}
