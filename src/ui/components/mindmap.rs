use ratatui::{backend::Backend, widgets::Paragraph, style::{Style, Color, Modifier}, Frame};
use crate::ui::layout::Rect;
use crate::state::AppState;
use crate::ui::animate;

/// Render a very simple mindmap view showing root node labels.
/// Recently selected nodes fade with a glow trail effect.
pub fn render_mindmap<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState, tick: u64) {
    let mut y = area.y;
    for id in &state.root_nodes {
        if let Some(node) = state.nodes.get(id) {
            let mut style = Style::default().fg(Color::White);
            if Some(*id) == state.selected {
                style = Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD | Modifier::UNDERLINED);
            } else if let Some(age) = state.selection_age(*id) {
                style = animate::glow_trail(Color::Yellow, tick, age, 900);
            }
            let rect = Rect::new(area.x + 1, y, area.width.saturating_sub(2), 1);
            f.render_widget(Paragraph::new(node.label.clone()).style(style), rect);
            y = y.saturating_add(1);
        }
    }
}
