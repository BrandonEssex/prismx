use ratatui::{backend::Backend, layout::Rect, style::{Color, Style}, widgets::Paragraph, Frame};

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

pub struct BeamX {
    pub tick: u64,
    pub enabled: bool,
    pub style: BeamXStyle,
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

        let pulse = self.style.pulse;
        let prism = pulse[(self.tick as usize) % pulse.len()];
        let x = area.right().saturating_sub(7);
        let y = area.top();

        let style_border = Style::default().fg(self.style.border_color);
        let style_status = Style::default().fg(self.style.status_color);
        let style_prism = Style::default().fg(self.style.prism_color);

        f.render_widget(Paragraph::new(self.style.top_left).style(style_border), Rect::new(x, y, 1, 1));
        f.render_widget(Paragraph::new(self.style.top_right).style(style_border), Rect::new(x + 6, y, 1, 1));

        f.render_widget(Paragraph::new(self.style.left).style(style_status), Rect::new(x + 1, y + 1, 1, 1));
        f.render_widget(Paragraph::new(prism).style(style_prism), Rect::new(x + 3, y + 1, 1, 1));
        f.render_widget(Paragraph::new(self.style.right).style(style_status), Rect::new(x + 5, y + 1, 1, 1));

        f.render_widget(Paragraph::new(self.style.bottom_left).style(style_border), Rect::new(x, y + 2, 1, 1));
        f.render_widget(Paragraph::new(self.style.bottom_right).style(style_border), Rect::new(x + 6, y + 2, 1, 1));
    }
}
