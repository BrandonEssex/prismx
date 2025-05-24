use ratatui::{backend::Backend, layout::Rect, style::{Color, Style, Modifier}, widgets::Paragraph, Frame};

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
        let frame = tick % 24;
        let x = area.right().saturating_sub(7);
        let y = area.top();

        let border = Style::default().fg(self.style.border_color);
        let status = Style::default().fg(self.style.status_color);
        let prism = Style::default().fg(self.style.prism_color);

        match frame {
            0..=2 => {
                // Entry arrow
                f.render_widget(Paragraph::new("⇙").style(status), Rect::new(x + 6, y, 1, 1));
            }
            3..=4 => {
                f.render_widget(Paragraph::new("✦").style(prism), Rect::new(x + 3, y + 1, 1, 1));
            }
            5..=7 => {
                f.render_widget(Paragraph::new("X").style(prism.add_modifier(Modifier::BOLD)), Rect::new(x + 3, y + 1, 1, 1));
            }
            8..=10 => {
                f.render_widget(Paragraph::new("X").style(prism.add_modifier(Modifier::BOLD)), Rect::new(x + 3, y + 1, 1, 1));
                f.render_widget(Paragraph::new("⬉").style(border), Rect::new(x, y, 1, 1));
                f.render_widget(Paragraph::new("⬊").style(border), Rect::new(x + 6, y + 2, 1, 1));
            }
            11..=14 => {
                let b = border.add_modifier(Modifier::BOLD);
                f.render_widget(Paragraph::new("X").style(prism.add_modifier(Modifier::BOLD)), Rect::new(x + 3, y + 1, 1, 1));
                f.render_widget(Paragraph::new("⬉").style(b), Rect::new(x, y, 1, 1));
                f.render_widget(Paragraph::new("⬊").style(b), Rect::new(x + 6, y + 2, 1, 1));
            }
            15..=19 => {
                let dim_border = border.add_modifier(Modifier::DIM);
                let dim_status = status.add_modifier(Modifier::DIM);
                f.render_widget(Paragraph::new("⇙").style(dim_status), Rect::new(x + 6, y, 1, 1));
                f.render_widget(Paragraph::new("⬉").style(dim_border), Rect::new(x, y, 1, 1));
                f.render_widget(Paragraph::new("⬊").style(dim_border), Rect::new(x + 6, y + 2, 1, 1));
            }
            _ => {
                // Idle / reset
                f.render_widget(Paragraph::new("X").style(prism), Rect::new(x + 3, y + 1, 1, 1));
            }
        }
    }
}
