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
        let frame = tick % 12;
        let beam_phase = frame % 4;
        let center_phase = tick % 8;
        let x = area.right().saturating_sub(7);
        let y = area.top();

        let border = Style::default().fg(self.style.border_color);
        let status = Style::default().fg(self.style.status_color);

        let entry_glyph = "⇙";
        let exit_tl = "⬉";
        let exit_br = "⬊";

        let bright_status = Style::default()
            .fg(bright_color(self.style.status_color))
            .add_modifier(Modifier::BOLD);
        let entry_style = match beam_phase {
            0 => status.add_modifier(Modifier::DIM),
            1 => status,
            2 => status.add_modifier(Modifier::BOLD),
            _ => bright_status,
        };

        let bright_border = Style::default().fg(bright_color(self.style.border_color));
        let exit_style = match beam_phase {
            0 => border.add_modifier(Modifier::DIM),
            1 => border,
            2 => bright_border.add_modifier(Modifier::BOLD),
            _ => border.add_modifier(Modifier::DIM),
        };

        let center_glyph = match center_phase {
            0 => "·",
            1 => "✦",
            2 => "◆",
            3 => "✸",
            4 => "x",
            5 => "X",
            6 => "✶",
            _ => "✦",
        };

        let prism_color_cycle = match tick % 3 {
            0 => self.style.prism_color,
            1 => Color::Cyan,
            _ => Color::Red,
        };

        let center_style = if center_glyph == "X" {
            Style::default()
                .fg(bright_color(prism_color_cycle))
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(prism_color_cycle)
        };

        // Exit beams around the center
        f.render_widget(Paragraph::new(exit_tl).style(exit_style), Rect::new(x, y, 1, 1));
        f.render_widget(Paragraph::new(exit_tl).style(exit_style), Rect::new(x + 1, y + 1, 1, 1));
        f.render_widget(Paragraph::new(exit_br).style(exit_style), Rect::new(x + 5, y + 3, 1, 1));
        f.render_widget(Paragraph::new(exit_br).style(exit_style), Rect::new(x + 6, y + 4, 1, 1));

        // Pulse border corners when beams brighten
        if beam_phase == 2 {
            f.render_widget(Paragraph::new("┏").style(bright_border), Rect::new(area.x, area.y, 1, 1));
            let br_x = area.right() - 1;
            let br_y = area.bottom() - 1;
            f.render_widget(Paragraph::new("┛").style(bright_border), Rect::new(br_x, br_y, 1, 1));
        }

        // Center pulse
        f.render_widget(Paragraph::new(center_glyph).style(center_style), Rect::new(x + 3, y + 2, 1, 1));

        // Entry beams stacked in the top-right corner
        f.render_widget(Paragraph::new(entry_glyph).style(entry_style), Rect::new(x + 6, y, 1, 1));
        f.render_widget(Paragraph::new(entry_glyph).style(entry_style), Rect::new(x + 6, y + 1, 1, 1));
    }
}
