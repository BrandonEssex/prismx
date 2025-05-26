use ratatui::{backend::Backend, layout::Rect, style::Style, widgets::{Block, Borders, Paragraph}, Frame};
use crate::beamx::render_full_border;
use crate::state::AppState;
use crate::ui::beamx::{BeamX, BeamXStyle, BeamXMode, BeamXAnimationMode};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn render_triage<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    let style = state.beam_style_for_mode("triage");
    let block = Block::default().title("Triage Panel").borders(Borders::NONE)
        .style(Style::default().fg(ratatui::style::Color::Red));

    let content = Paragraph::new(
        "• GemX rendering: OK\n\
         • Node editing: OK\n\
         • Zen scroll: OK\n\
         • Triage display: Working"
    );

    f.render_widget(block, area);
    f.render_widget(content, Rect::new(area.x + 2, area.y + 1, area.width - 4, area.height - 2));
    render_full_border(f, area, &style, true, false);
    let tick = (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() / 300) as u64;
    let mut bx_style = BeamXStyle::from(BeamXMode::Triage);
    let (b, s, p) = state.beamx_panel_theme.palette();
    bx_style.border_color = b;
    bx_style.status_color = s;
    bx_style.prism_color = p;
    let beamx = BeamX {
        tick,
        enabled: state.beamx_panel_visible,
        mode: BeamXMode::Triage,
        style: bx_style,
        animation: BeamXAnimationMode::PulseEntryRadiate,
    };
    beamx.render(f, area);
}
