use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use crate::state::AppState;

pub fn render_mindmap(f: &mut Frame, app: &AppState, area: Rect) {
    let items: Vec<ListItem> = app.node_tree.nodes.iter().map(|node| {
        let prefix = if node.editing { "[editing] " } else { "" };
        let line = format!("{}{}", prefix, node.label);
        ListItem::new(line)
    }).collect();

    let mut state = ListState::default();
    state.select(Some(app.node_tree.selected_index));

    let list = List::new(items)
        .block(Block::default().title("Mindmap").borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        );

    f.render_stateful_widget(list, area, &mut state);
}
