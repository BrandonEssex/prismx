use ratatui::prelude::*;

pub fn render_unknown_mode<B: Backend>(f: &mut Frame<B>, area: Rect) {
    use ratatui::widgets::Paragraph;
    let fallback = Paragraph::new("Unknown mode");
    f.render_widget(fallback, area);
}
