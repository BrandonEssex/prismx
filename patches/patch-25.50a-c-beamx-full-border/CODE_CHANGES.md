## Code Changes

- In `src/beamx.rs`, define:
  ```rust
  pub struct BeamStyle {
      pub border_color: Color,
      pub beam_left: Color,
      pub beam_right: Color,
      pub prism: Color,
  }

  pub fn style_for_mode(mode: &str) -> BeamStyle {
      match mode {
          "gemx" => BeamStyle {
              border_color: Color::Magenta,
              beam_left: Color::Cyan,
              beam_right: Color::Magenta,
              prism: Color::Yellow,
          },
          "zen" => BeamStyle {
              border_color: Color::White,
              beam_left: Color::White,
              beam_right: Color::White,
              prism: Color::Gray,
          },
          "triage" => BeamStyle {
              border_color: Color::Red,
              beam_left: Color::Yellow,
              beam_right: Color::Red,
              prism: Color::White,
          },
          "settings" => BeamStyle {
              border_color: Color::Blue,
              beam_left: Color::Cyan,
              beam_right: Color::Blue,
              prism: Color::White,
          },
          _ => BeamStyle {
              border_color: Color::Gray,
              beam_left: Color::Gray,
              beam_right: Color::Gray,
              prism: Color::Gray,
          },
      }
  }
In render_beam_logo(frame, area, style):
Draw:
╱ using style.beam_left
╲ using style.beam_right
◉ at center using style.prism
Center horizontally
In render_full_border(frame, area, style):
Draw top/bottom: ┏━━┓ + ┗━━┛
Draw verticals: ┃ left/right
Use style.border_color
In render_gemx() and other modules:
Import style_for_mode(state.mode)
Call render_beam_logo(..., style)
Call render_full_border(..., style)
