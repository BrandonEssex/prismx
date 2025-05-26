use ratatui::{prelude::*, widgets::{Block, Paragraph}, Frame};

use crate::state::AppState;
use crate::render::traits::Renderable;
use crate::screen::gemx::render_gemx;
use crate::layout::BASE_SPACING_X;
use crate::gemx::layout::{apply_layout, set_mode};
use crate::config::theme::ThemeConfig;

/// Wrapper implementing [`Renderable`] for the GemX screen.
pub struct GemxRenderer<'a> {
    pub state: &'a mut AppState,
}

impl<'a> GemxRenderer<'a> {
    pub fn new(state: &'a mut AppState) -> Self {
        let cfg = ThemeConfig::load();
        set_mode(cfg.layout_mode());
        Self { state }
    }
}

impl<'a> Renderable for GemxRenderer<'a> {
    fn render_frame<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        if self.state.mindmap_lanes {
            draw_lanes(f, area);
        }
        // Apply layout before rendering
        apply_layout(&mut self.state.nodes, &self.state.root_nodes);
        // Render main GemX view
        render_gemx(f, area, self.state);
    }

    fn tick(&mut self) {
        crate::triage::logic::update_pipeline(self.state);
    }
}

fn draw_lanes<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let theme = ThemeConfig::load();
    let lane_color = theme.dim_color();
    let lane_style = Style::default().bg(lane_color);
    let lane_height = 3u16;

    let mut toggle = false;
    let mut y = 0u16;
    while y < area.height {
        if toggle {
            let h = lane_height.min(area.height - y);
            let rect = Rect::new(area.x, area.y + y, area.width, h);
            f.render_widget(Block::default().style(lane_style), rect);
        }
        toggle = !toggle;
        y += lane_height;
    }

    let dot = Paragraph::new("┆").style(Style::default().fg(lane_color));
    let step = BASE_SPACING_X as u16;
    let mut x = step;
    while x < area.width.saturating_sub(1) {
        for row in 0..area.height {
            let rect = Rect::new(area.x + x, area.y + row, 1, 1);
            f.render_widget(dot.clone(), rect);
        }
        x += step;
    }
}

