use ratatui::{
    backend::Backend,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::ui::layout::Rect;
use crate::plugin::registry::{registry, PluginStatus};

/// Render a panel listing all loaded plugins.
pub fn render_plugin_status<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let entries: Vec<PluginStatus> = registry();
    let lines: Vec<String> = if entries.is_empty() {
        vec!["No plugins loaded".to_string()]
    } else {
        entries
            .iter()
            .map(|p| {
                if let Some(ref path) = p.path {
                    format!("{} v{} ({})", p.name, p.version, path)
                } else {
                    format!("{} v{}", p.name, p.version)
                }
            })
            .collect()
    };

    let para = Paragraph::new(lines.join("\n"))
        .block(Block::default().title("Loaded Plugins").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));
    f.render_widget(para, area);
}
