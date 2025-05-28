use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Style, Modifier},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::config::theme::ThemeConfig;
use crate::state::AppState;
use crate::ui::animate::shimmer;

/// Render the animated title bar for the Mindmap module.
/// The animation runs for a few frames when the module
/// is entered and gently pulses using a shimmer effect.
pub fn render_title_bar<B: Backend>(f: &mut Frame<B>, area: Rect, state: &mut AppState) {
    let accent = ThemeConfig::load().accent_color();
    let tick = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() / 300;

    let style = if state.mindmap_title_frames > 0 {
        shimmer(accent, tick as u64)
    } else {
        Style::default().fg(accent)
    };

    // Center the label on the top border
    let label = "Mindmap";
    let width = unicode_width::UnicodeWidthStr::width(label) as u16;
    let x = area.x + area.width.saturating_sub(width) / 2;
    let rect = Rect::new(x, area.y, width, 1);
    let line = Line::from(vec![Span::styled(label, style.add_modifier(Modifier::BOLD))]);
    f.render_widget(Paragraph::new(line), rect);

    if state.mindmap_title_frames > 0 && std::env::var("PRISMX_TEST").is_err() {
        state.mindmap_title_frames -= 1;
    }
}
