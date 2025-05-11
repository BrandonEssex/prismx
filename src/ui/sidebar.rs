use ratatui::{
    backend::Backend,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::state::SidebarView;

pub fn render_sidebar(f: &mut Frame<'_>, area: Rect, view: &SidebarView) {
    let title = match view {
        SidebarView::Meta => "Metadata",
        SidebarView::Outline => "Outline",
        SidebarView::Tags => "Tags",
        SidebarView::None => return,
    };

    let content = format!("Sidebar View: {title}");

    let block = Paragraph::new(content)
        .block(Block::default().title(title).borders(Borders::ALL));

    f.render_widget(block, area);
}