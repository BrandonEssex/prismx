use ratatui::{
    layout::Rect,
    style::{Style, Modifier},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::state::AppState;

pub fn render_sidebar(f: &mut Frame, _app: &AppState, area: Rect) {
    let items = vec![
        ListItem::new("Mindmap"),
        ListItem::new("Dashboard"),
        ListItem::new("Plugins"),
    ];

    let list = List::new(items)
        .block(Block::default().title("Views").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_widget(list, area);
}
