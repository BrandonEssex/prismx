use ratatui::{backend::Backend, layout::Rect, style::Style, widgets::{Block, Borders, Paragraph}, Frame};
use crate::beamx::{render_full_border, style_for_mode};
use crate::ui::beamx::{BeamX, BeamXStyle, BeamXMode, BeamXAnimationMode};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn render_triage<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let style = style_for_mode("triage");
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
    let beamx = BeamX {
        tick,
        enabled: true,
        mode: BeamXMode::Triage,
        style: BeamXStyle::from(BeamXMode::Triage),
        animation: BeamXAnimationMode::PulseEntryRadiate,
    };
    beamx.render(f, area);
}
