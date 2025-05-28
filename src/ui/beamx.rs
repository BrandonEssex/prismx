use ratatui::{backend::Backend, style::{Color, Style, Modifier}, widgets::Paragraph, Frame};
use crate::ui::layout::Rect;

pub(crate) fn bright_color(c: Color) -> Color {
    use Color::*;
    match c {
        Black => DarkGray,
        Red => LightRed,
        Green => LightGreen,
        Yellow => LightYellow,
        Blue => LightBlue,
        Magenta => LightMagenta,
        Cyan => LightCyan,
        Gray | DarkGray => White,
        _ => c,
    }
}

pub(crate) fn render_glyph<B: Backend>(f: &mut Frame<B>, x: u16, y: u16, glyph: &str, style: Style) {
    f.render_widget(Paragraph::new(glyph).style(style), Rect::new(x, y, 1, 1));
}

/// Visual style and glyph configuration for [`BeamX`].
pub struct BeamXStyle {
    pub border_color: Color,
    pub status_color: Color,
    pub prism_color: Color,
    pub top_left: &'static str,
    pub bottom_left: &'static str,
    pub left: &'static str,
    pub right: &'static str,
    pub top_right: &'static str,
    pub bottom_right: &'static str,
    pub pulse: &'static [&'static str],
}

/// Operational mode for [`BeamX`].
#[derive(Copy, Clone)]
pub enum BeamXMode {
    Default,
    Zen,
    Triage,
    Spotlight,
    Settings,
    Debug,
}

/// Animation themes for [`BeamX`].
#[derive(Copy, Clone)]
pub enum BeamXAnimationMode {
    PulseEntryRadiate,
    FadeOut,
    // Future: ZenFade,
    // Future: SpotlightBlink,
    // Future: DebugFlash,
}

impl Default for BeamXAnimationMode {
    fn default() -> Self {
        BeamXAnimationMode::PulseEntryRadiate
    }
}

pub struct BeamX {
    pub tick: u64,
    pub enabled: bool,
    pub mode: BeamXMode,
    pub style: BeamXStyle,
    pub animation: BeamXAnimationMode,
}

impl Default for BeamXStyle {
    fn default() -> Self {
        BeamXStyle::from(BeamXMode::Default)
    }
}

impl From<BeamXMode> for BeamXStyle {
    fn from(mode: BeamXMode) -> Self {
        const DEFAULT_PULSE: [&str; 12] = ["·", "✦", "◆", "✦", "·", "x", "X", "x", "·", "✦", "◆", "✦"];
        const ZEN_PULSE: [&str; 5] = ["∙", "◦", "●", "◦", "∙"];

        match mode {
            BeamXMode::Zen => Self {
                border_color: Color::Blue,
                status_color: Color::Gray,
                prism_color: Color::White,
                top_left: "∙",
                bottom_left: "∙",
                left: "◦",
                right: "◦",
                top_right: "∙",
                bottom_right: "∙",
                pulse: &ZEN_PULSE,
            },
            BeamXMode::Triage => Self {
                border_color: Color::Red,
                status_color: Color::Yellow,
                prism_color: Color::White,
                top_left: "⚠",
                bottom_left: "⚠",
                left: "→",
                right: "←",
                top_right: "⚠",
                bottom_right: "⚠",
                pulse: &DEFAULT_PULSE,
            },
            BeamXMode::Spotlight => Self {
                border_color: Color::Magenta,
                status_color: Color::White,
                prism_color: Color::White,
                top_left: "»",
                bottom_left: "»",
                left: ">",
                right: "<",
                top_right: "«",
                bottom_right: "«",
                pulse: &DEFAULT_PULSE,
            },
            BeamXMode::Settings => Self {
                border_color: Color::Green,
                status_color: Color::White,
                prism_color: Color::White,
                top_left: "-",
                bottom_left: "-",
                left: ">",
                right: "<",
                top_right: "-",
                bottom_right: "-",
                pulse: &DEFAULT_PULSE,
            },
            BeamXMode::Debug => Self {
                border_color: Color::Yellow,
                status_color: Color::Yellow,
                prism_color: Color::White,
                top_left: "⬊",
                bottom_left: "⬈",
                left: "⥤",
                right: "⥢",
                top_right: "⬋",
                bottom_right: "⬉",
                pulse: &DEFAULT_PULSE,
            },
            BeamXMode::Default => Self {
                border_color: Color::Magenta,
                status_color: Color::Cyan,
                prism_color: Color::White,
                top_left: "⬊",
                bottom_left: "⬈",
                left: "⥤",
                right: "⥢",
                top_right: "⬋",
                bottom_right: "⬉",
                pulse: &DEFAULT_PULSE,
            },
        }
    }
}

impl BeamX {
    pub fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        if !self.enabled {
            return;
        }

        if matches!(self.mode, BeamXMode::Debug) {
            tracing::debug!("Shimmer bounds: {} -> {}", area.x, area.x + area.width);
        }

        match self.animation {
            BeamXAnimationMode::PulseEntryRadiate => {
                self.render_pulse_entry_radiate(f, area, self.tick);
            }
            BeamXAnimationMode::FadeOut => {
                self.render_frame(f, area, self.tick);
                self.render_shimmer(f, area);
            }
        }
    }

    fn render_pulse_entry_radiate<B: Backend>(
        &self,
        f: &mut Frame<B>,
        area: Rect,
        tick: u64,
    ) {
        let frame = tick % 24;
        let x = area.right().saturating_sub(7);
        let y = area.top();

        let border = Style::default().fg(self.style.border_color);
        let status = Style::default().fg(self.style.status_color);
        let prism = Style::default().fg(self.style.prism_color);

        match frame {
            0..=2 => {
                render_glyph(f, x + 6, y + 0, "⇙", status);
            }
            3..=4 => {
                render_glyph(f, x + 3, y + 1, "✦", prism);
            }
            5..=7 => {
                render_glyph(
                    f,
                    x + 3,
                    y + 1,
                    "X",
                    prism.add_modifier(Modifier::BOLD),
                );
            }
            8..=10 => {
                render_glyph(
                    f,
                    x + 3,
                    y + 1,
                    "X",
                    prism.add_modifier(Modifier::BOLD),
                );
                render_glyph(f, x + 0, y + 0, "⬉", border);
                render_glyph(f, x + 6, y + 2, "⬊", border);
            }
            11..=14 => {
                let b = border.add_modifier(Modifier::BOLD);
                render_glyph(
                    f,
                    x + 3,
                    y + 1,
                    "X",
                    prism.add_modifier(Modifier::BOLD),
                );
                render_glyph(f, x + 0, y + 0, "⬉", b);
                render_glyph(f, x + 6, y + 2, "⬊", b);
            }
            15..=19 => {
                let dim_border = border.add_modifier(Modifier::DIM);
                let dim_status = status.add_modifier(Modifier::DIM);
                render_glyph(f, x + 6, y + 0, "⇙", dim_status);
                render_glyph(f, x + 0, y + 0, "⬉", dim_border);
                render_glyph(f, x + 6, y + 2, "⬊", dim_border);
            }
            _ => {
                render_glyph(f, x + 3, y + 1, "X", prism);
            }
        }
    }

    fn render_frame<B: Backend>(&self, f: &mut Frame<B>, area: Rect, tick: u64) {
        super::render::draw_beam(f, area, tick, &self.style);
    }

    fn render_shimmer<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let start_x = area.right().saturating_sub(12);
        let end_x = area.x + area.width;
        let y = area.y;
        for sx in start_x..end_x {
            if sx == area.x + area.width - 1 && y == area.y + area.height - 1 {
                continue; // Prevent corner glyph artifact
            }
            let mut style = Style::default().fg(self.style.status_color);
            if sx + 1 >= end_x {
                style = style.add_modifier(Modifier::DIM);
            }
            render_glyph(f, sx, y, "━", style);
        }
    }
}
