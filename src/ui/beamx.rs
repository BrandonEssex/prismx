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

/// Convenience helper for modules to render BeamX without constructing
/// a [`BeamX`] instance.
pub fn render_beam<B: Backend>(f: &mut Frame<B>, area: Rect, tick: u64, style: &BeamXStyle) {
    super::render::draw_beam(f, area, tick, style);
}

pub fn render_beam_for_mode<B: Backend>(
    f: &mut Frame<B>,
    area: Rect,
    tick: u64,
    mode: BeamXMode,
) {
    let style = BeamXStyle::from(mode);
    render_beam(f, area, tick, &style);
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
                border_color: Color::Yellow,
                status_color: Color::Yellow,
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
                top_right: "↙",
                bottom_right: "↖",
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
                top_right: "↙",
                bottom_right: "↖",
                pulse: &DEFAULT_PULSE,
            },
        }
    }
}

#[derive(Copy, Clone)]
pub enum InsertCursorKind {
    Child,
    Sibling,
}

/// Render a pulsing cursor at the given position using Zen-style animation.
pub fn render_insert_cursor<B: Backend>(
    f: &mut Frame<B>,
    pos: (u16, u16),
    tick: u64,
    kind: InsertCursorKind,
    style: &BeamXStyle,
) {
    use crate::ui::animate::breath_style;
    let glyph = match kind {
        InsertCursorKind::Child => "│",
        InsertCursorKind::Sibling => crate::ui::animate::pulse(style.pulse, tick),
    };
    let color = match kind {
        InsertCursorKind::Child => style.border_color,
        InsertCursorKind::Sibling => style.status_color,
    };
    let s = breath_style(color, tick);
    render_glyph(f, pos.0, pos.1, glyph, s);
}

