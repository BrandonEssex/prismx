use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Style, Color, Modifier},
    text::Span,
    widgets::Paragraph,
    Frame,
};
use crate::render::traits::{Renderable, RenderFrame};
use crate::ui::animate;
use crate::state::AppState;

/// Render a mindmap view showing root node labels with animated glow trail.
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

/// Draw an L-shaped connector between parent and child, avoiding label overlap.
pub fn draw_connector<B: Backend>(
    f: &mut Frame<B>,
    area: Rect,
    start: (u16, u16),
    end: (u16, u16),
    avoid_width: u16,
) {
    let (sx, sy) = start;
    let (ex, ey) = end;

    if sx >= area.right() || ex >= area.right() {
        return;
    }

    let route_x = sx.saturating_sub(avoid_width.saturating_add(1));

    if sy <= ey {
        for y in sy + 1..=ey {
            if y >= area.bottom() || route_x >= area.right() {
                break;
            }
            f.render_widget(Paragraph::new("│"), Rect::new(route_x, y, 1, 1));
        }
    } else {
        for y in ey..sy {
            if y >= area.bottom() || route_x >= area.right() {
                break;
            }
            f.render_widget(Paragraph::new("│"), Rect::new(route_x, y, 1, 1));
        }
    }

    if ey < area.bottom() && route_x < area.right() {
        let corner = if ex >= route_x { "└" } else { "┘" };
        f.render_widget(Paragraph::new(corner), Rect::new(route_x, ey, 1, 1));
    }

    if route_x <= ex {
        for x in route_x + 1..=ex {
            if x >= area.right() || ey >= area.bottom() {
                break;
            }
            f.render_widget(Paragraph::new("─"), Rect::new(x, ey, 1, 1));
        }
    } else {
        for x in ex..route_x {
            if x >= area.right() || ey >= area.bottom() {
                break;
            }
            f.render_widget(Paragraph::new("─"), Rect::new(x, ey, 1, 1));
        }
    }
}

/// Demo renderer showing connector routing between two nodes.
pub fn render_demo<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let parent_pos = (area.x + 10, area.y + 2);
    let child_pos = (area.x + 20, area.y + 6);

    f.render_widget(Paragraph::new("Parent"), Rect::new(parent_pos.0, parent_pos.1, 6, 1));
    f.render_widget(Paragraph::new("Child"), Rect::new(child_pos.0, child_pos.1, 5, 1));

    draw_connector(f, area, parent_pos, child_pos, 6);
}

/// Wrapper implementing [`Renderable`] for the demo mindmap view.
pub struct MindmapDemo;

impl Renderable for MindmapDemo {
    fn render(&mut self, f: &mut RenderFrame<'_>, area: Rect) {
        render_demo(f, area);
    }
}
