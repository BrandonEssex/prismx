use crate::state::{AppState, SidebarView};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_sidebar(f: &mut Frame<'_>, area: Rect, view: &SidebarView) {
    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title(match view {
            SidebarView::Meta => "Metadata",
            SidebarView::Outline => "Outline",
            SidebarView::Triage => "Triage",
        });

    let paragraph = Paragraph::new("").block(block);
    f.render_widget(paragraph, area);
}