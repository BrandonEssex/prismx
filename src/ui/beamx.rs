use ratatui::{backend::Backend, layout::Rect, style::{Color, Style}, widgets::Paragraph, Frame};

pub struct BeamX {
    pub tick: u64,
    pub enabled: bool,
    pub style: BeamXStyle,
}

pub struct BeamXStyle {
    pub border_color: Color,
    pub status_color: Color,
    pub prism_color: Color,
}

impl Default for BeamXStyle {
    fn default() -> Self {
        Self {
            border_color: Color::LightCyan,
            status_color: Color::Magenta,
            prism_color: Color::White,
        }
    }
}

impl BeamX {
    pub fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        if !self.enabled {
            return;
        }

        let prism = match self.tick % 12 {
            0 => "·",
            1 => "✦",
            2 => "◆",
            3 => "✦",
            4 => "·",
            5 => "x",
            6 => "X",
            7 => "x",
            8 => "·",
            9 => "✦",
            10 => "◆",
            _ => "✦",
        };

        let x = area.right().saturating_sub(8);
        let y = area.top();

        let style_border = Style::default().fg(self.style.border_color);
        let style_status = Style::default().fg(self.style.status_color);
        let style_prism = Style::default().fg(self.style.prism_color);

        let tl = Paragraph::new("⬊").style(style_border);
        f.render_widget(tl, Rect::new(x, y, 1, 1));
        let tr = Paragraph::new("⬋").style(style_border);
        f.render_widget(tr, Rect::new(x + 6, y, 1, 1));

        let left = Paragraph::new("⥤").style(style_status);
        f.render_widget(left, Rect::new(x + 1, y + 1, 1, 1));
        let center = Paragraph::new(prism).style(style_prism);
        f.render_widget(center, Rect::new(x + 3, y + 1, 1, 1));
        let right = Paragraph::new("⥢").style(style_status);
        f.render_widget(right, Rect::new(x + 5, y + 1, 1, 1));

        let bl = Paragraph::new("⬈").style(style_border);
        f.render_widget(bl, Rect::new(x, y + 2, 1, 1));
        let br = Paragraph::new("⬉").style(style_border);
        f.render_widget(br, Rect::new(x + 6, y + 2, 1, 1));
    }
}
