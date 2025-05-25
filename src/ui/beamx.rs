use ratatui::{backend::Backend, layout::Rect, style::{Color, Style, Modifier}, widgets::Paragraph, Frame};

fn bright_color(c: Color) -> Color {
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
}

/// Animation themes for [`BeamX`].
#[derive(Copy, Clone)]
pub enum BeamXAnimationMode {
    PulseEntryRadiate,
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

        match self.animation {
            BeamXAnimationMode::PulseEntryRadiate => {
                self.render_frame(f, area, self.tick);
            }
        }
    }

    fn render_frame<B: Backend>(&self, f: &mut Frame<B>, area: Rect, tick: u64) {
        let frame8 = tick % 8;
        let frame12 = tick % 12;
        let x = area.right().saturating_sub(7);
        let y = area.top();

        let border = Style::default().fg(self.style.border_color);
        let status = Style::default().fg(self.style.status_color);
        let prism = Style::default().fg(self.style.prism_color);

        let entry_glyph = "⇙";
        let exit_tl = "⬉";
        let exit_br = "⬊";

        let bright_status = Style::default()
            .fg(bright_color(self.style.status_color))
            .add_modifier(Modifier::BOLD);
        let entry_style = match frame8 {
            0 => status.add_modifier(Modifier::DIM),
            1 => status,
            2 => status.add_modifier(Modifier::BOLD),
            3 => bright_status,
            4 => status,
            5 => status.add_modifier(Modifier::DIM),
            _ => status,
        };

        let bright_border = Style::default().fg(bright_color(self.style.border_color));
        let exit_style = match frame8 {
            0 => border.add_modifier(Modifier::DIM),
            1 => border,
            2 => border.add_modifier(Modifier::BOLD),
            3 => bright_border.add_modifier(Modifier::BOLD),
            4 => border,
            5 => border.add_modifier(Modifier::DIM),
            _ => border,
        };

        let center_glyph = match frame12 {
            0 => "·",
            1 => "✦",
            2 => "◆",
            3 => "✷",
            4 => "✸",
            5 => "x",
            6 => "X",
            7 => "✸",
            8 => "✷",
            9 => "◆",
            10 => "✦",
            _ => "·",
        };

        let center_style = if center_glyph == "X" {
            prism.add_modifier(Modifier::BOLD)
        } else {
            prism
        };

        // Exit beams around the center
        f.render_widget(Paragraph::new(exit_tl).style(exit_style), Rect::new(x, y, 1, 1));
        f.render_widget(Paragraph::new(exit_br).style(exit_style), Rect::new(x + 6, y + 2, 1, 1));

        // Pulse border corners when beams brighten
        if frame8 == 3 || frame8 == 4 {
            f.render_widget(Paragraph::new("┏").style(bright_border), Rect::new(area.x, area.y, 1, 1));
            let br_x = area.right() - 1;
            let br_y = area.bottom() - 1;
            f.render_widget(Paragraph::new("┛").style(bright_border), Rect::new(br_x, br_y, 1, 1));
        }

        // Center pulse
        f.render_widget(Paragraph::new(center_glyph).style(center_style), Rect::new(x + 3, y + 1, 1, 1));

        // Entry beam always appears in the top-right corner
        f.render_widget(Paragraph::new(entry_glyph).style(entry_style), Rect::new(x + 6, y, 1, 1));
    }
}
