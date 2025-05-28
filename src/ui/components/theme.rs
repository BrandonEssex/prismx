use ratatui::{
    backend::Backend,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use crate::ui::layout::Rect;

use crate::config_store::theme::ThemeConfig;

/// Render the theme editor panel.
/// This is a simple UI for adjusting brightness, contrast
/// and accent color with a basic preview of various modules.
pub fn render_theme_editor<B: Backend>(f: &mut Frame<B>, area: Rect, cfg: &ThemeConfig) {
    let block = Block::default().title("Theme Editor").borders(Borders::ALL);
    f.render_widget(block, area);

    let inner = Rect::new(area.x + 1, area.y + 1, area.width - 2, area.height - 2);
    let mut lines = Vec::new();
    lines.push(Line::from(Span::raw(format!("Text Brightness: {:.1}", cfg.text_brightness))));
    lines.push(Line::from(Span::raw(format!("Border Contrast: {:.1}", cfg.border_contrast))));
    lines.push(Line::from(Span::raw(format!("Accent Color: {}", cfg.accent_color))));
    lines.push(Line::from(Span::raw("")));
    lines.push(Line::from(Span::styled(
        "Zen Journal Preview",
        Style::default().fg(Color::White),
    )));
    lines.push(Line::from(Span::styled(
        "GemX Node Preview",
        Style::default().fg(Color::Cyan),
    )));
    lines.push(Line::from(Span::styled(
        "Spotlight Prompt Preview",
        Style::default().fg(Color::Yellow),
    )));

    let para = Paragraph::new(lines).wrap(Wrap { trim: false });
    f.render_widget(para, inner);
}

