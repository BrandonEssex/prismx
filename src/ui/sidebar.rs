// FINAL FULL FILE DELIVERY
// Filename: /src/ui/sidebar.rs
// File Delivery Progress: 10/∞ FINAL FILES delivered

use ratatui::{Frame};
use ratatui::layout::Rect;
use crate::state::AppState;

pub trait SidebarPanel {
    fn title(&self) -> &'static str;
    fn render<B: ratatui::backend::Backend>(&self, f: &mut Frame<B>, area: Rect, app_state: &AppState);
}

pub struct MetaPanel;
impl SidebarPanel for MetaPanel {
    fn title(&self) -> &'static str {
        "Meta"
    }
    fn render<B: ratatui::backend::Backend>(&self, f: &mut Frame<B>, area: Rect, _state: &AppState) {
        let block = ratatui::widgets::Block::default().title("Metadata").borders(ratatui::widgets::Borders::ALL);
        f.render_widget(block, area);
    }
}

pub struct OutlinePanel;
impl SidebarPanel for OutlinePanel {
    fn title(&self) -> &'static str {
        "Outline"
    }
    fn render<B: ratatui::backend::Backend>(&self, f: &mut Frame<B>, area: Rect, _state: &AppState) {
        let block = ratatui::widgets::Block::default().title("Outline").borders(ratatui::widgets::Borders::ALL);
        f.render_widget(block, area);
    }
}

pub fn render_sidebar<B: ratatui::backend::Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    let panels: Vec<Box<dyn SidebarPanel>> = vec![
        Box::new(MetaPanel),
        Box::new(OutlinePanel),
    ];
    let index = state.active_sidebar_tab % panels.len();
    panels[index].render(f, area, state);
}