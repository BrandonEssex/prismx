## Code Changes

- Create `src/ui/beamx.rs`:

```rust
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
        if !self.enabled { return; }

        let prism = match self.tick % 12 {
            0 => "·", 1 => "✦", 2 => "◆", 3 => "✦",
            4 => "·", 5 => "x", 6 => "X", 7 => "x",
            8 => "·", 9 => "✦", 10 => "◆", _ => "✦",
        };

        let beam_x = area.right().saturating_sub(8);
        let beam_y = area.top();

        let buf = f.buffer_mut();

        buf.set_string(beam_x,     beam_y,     "⬊", Style::default().fg(self.style.border_color));
        buf.set_string(beam_x + 6, beam_y,     "⬋", Style::default().fg(self.style.border_color));
        buf.set_string(beam_x + 1, beam_y + 1, "⥤", Style::default().fg(self.style.status_color));
        buf.set_string(beam_x + 3, beam_y + 1, prism, Style::default().fg(self.style.prism_color));
        buf.set_string(beam_x + 5, beam_y + 1, "⥢", Style::default().fg(self.style.status_color));
        buf.set_string(beam_x,     beam_y + 2, "⬈", Style::default().fg(self.style.border_color));
        buf.set_string(beam_x + 6, beam_y + 2, "⬉", Style::default().fg(self.style.border_color));
    }
}
In each module (render_gemx, render_zen, etc):
Add:
use crate::ui::beamx::BeamX;

let beamx = BeamX {
    tick: app.tick,
    enabled: true,
    style: BeamXStyle::default(),
};
beamx.render(f, area);
Add mod beamx; to ui/mod.rs
