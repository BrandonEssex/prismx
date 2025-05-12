use ratatui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_plugin_graph<B: Backend>(
    f: &mut Frame<B>,
    area: Rect,
    plugin_names: &[&str],
    links: &[(usize, usize)],
) {
    let layout = Layout::default()
        .constraints([Constraint::Percentage(100)])
        .split(area);

    let mut graph_string = String::new();
    for (i, &name) in plugin_names.iter().enumerate() {
        graph_string.push_str(&format!("({}) {}\n", i, name));
    }
    graph_string.push_str("\nLinks:\n");
    for (from, to) in links.iter() {
        graph_string.push_str(&format!("{} → {}\n", from, to));
    }

    let paragraph = Paragraph::new(graph_string)
        .block(
            Block::default()
                .title("Plugin Graph")
                .borders(Borders::ALL)
                .border_style(
                    Style::default()
                        .fg(Color::LightBlue)
                        .add_modifier(Modifier::BOLD),
                ),
        )
        .style(Style::default().fg(Color::White));

    f.render_widget(paragraph, layout[0]);
}
