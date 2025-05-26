use ratatui::prelude::*;
use crate::state::{AppState, ZenViewMode};
use crate::render::zen::render_zen_journal;
use crate::canvas::prism::render_prism;

pub fn render_zen<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    match state.zen_view_mode {
        ZenViewMode::Journal => render_zen_journal(f, area, state),
        ZenViewMode::Classic => render_classic(f, area, state),
        ZenViewMode::Summary => render_zen_journal(f, area, state),
        ZenViewMode::Split => {
            let mid = area.width / 2;
            let left = Rect { x: area.x, y: area.y, width: mid, height: area.height };
            let right = Rect { x: area.x + mid, y: area.y, width: area.width - mid, height: area.height };
            render_classic(f, left, state);
            render_zen_journal(f, right, state);
        }
    }
    render_prism(f, area);
}

fn render_classic<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    use ratatui::widgets::{Paragraph, Block, Borders};
    use ratatui::style::Style;
    use ratatui::text::Line;

    let mut lines = Vec::new();
    for line in &state.zen_buffer {
        lines.push(Line::from(line.as_str()));
    }
    let para = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default());
    f.render_widget(para, area);
    render_prism(f, area);
}
