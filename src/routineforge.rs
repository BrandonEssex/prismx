use ratatui::{
    backend::CrosstermBackend,
    layout::Rect,
    Frame,
    widgets::{Block, Borders, Paragraph},
    text::Line,
};
use crate::beamx::{render_full_border, style_for_mode};
use crate::ui::beamx::{BeamX, BeamXStyle, BeamXMode, BeamXAnimationMode};
use crate::state::AppState;
use std::io::Stdout;

type PluginFrame<'a> = Frame<'a, CrosstermBackend<Stdout>>;

use std::time::{SystemTime, UNIX_EPOCH};

pub fn render_triage_panel(f: &mut PluginFrame<'_>, area: Rect, state: &mut AppState) {
    let style = style_for_mode("triage");
    let tasks = vec![
        Line::from("[ ] Design new node engine"),
        Line::from("[x] Fix dashboard overflow"),
        Line::from("[ ] Write slash command spec"),
    ];

    let block = Block::default()
        .title("RoutineForge – Inbox")
        .borders(Borders::NONE);

    let paragraph = Paragraph::new(tasks).block(block);
    f.render_widget(paragraph, area);
    render_full_border(f, area, &style, true);
    let tick = (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() / 300) as u64;
    let beamx = BeamX {
        tick,
        enabled: true,
        style: BeamXStyle::from(BeamXMode::Triage),
        animation: BeamXAnimationMode::PulseEntryRadiate,
    };
    beamx.render(f, area);

    let plugin_area = Rect::new(area.x + 1, area.y + 1, area.width - 2, area.height - 4);
    state.plugin_host.render_all(f, plugin_area);
}
