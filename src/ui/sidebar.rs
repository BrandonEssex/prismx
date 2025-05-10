use ratatui::{Frame};
use ratatui::layout::Rect;
use crate::state::AppState;
use ratatui::widgets::{Block, Borders};

pub enum SidebarPanelKind {
    Meta,
    Outline,
}

pub fn render_sidebar(f: &mut Frame, area: Rect, state: &AppState) {
    match get_active_panel(state) {
        SidebarPanelKind::Meta => {
            let block = Block::default().title("Metadata").borders(Borders::ALL);
            f.render_widget(block, area);
        }
        SidebarPanelKind::Outline => {
            let block = Block::default().title("Outline").borders(Borders::ALL);
            f.render_widget(block, area);
        }
    }
}

fn get_active_panel(_state: &AppState) -> SidebarPanelKind {
    SidebarPanelKind::Meta
}