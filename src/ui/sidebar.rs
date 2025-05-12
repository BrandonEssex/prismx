// src/ui/sidebar.rs

use crate::state::SidebarView;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::text::{Line, Span};
use ratatui::style::Style;
use ratatui::Frame;

pub fn render_sidebar_panel(frame: &mut Frame<'_>, area: Rect, sidebar: &SidebarView) {
    let (title, lines) = match sidebar {
        SidebarView::Help => (
            "Help",
            vec![
                Line::from("q - Quit"),
                Line::from("Tab - Next Section"),
                Line::from("Ctrl+Z - Zen Mode"),
            ],
        ),
        SidebarView::Plugins => ("Plugins", vec![Line::from("Plugin list here.")]),
        SidebarView::Triage => ("Triage", vec![Line::from("Incoming items...")]),
        SidebarView::Scratchpad => ("Scratchpad", vec![Line::from("Notes...")]),
        SidebarView::Config => ("Config", vec![Line::from("Settings...")]),
        SidebarView::Hidden => ("", vec![]),
    };

    let block = Block::default().title(title).borders(Borders::ALL);

    let paragraph = Paragraph::new(lines)
        .block(block)
        .style(Style::default());

    frame.render_widget(paragraph, area);
}