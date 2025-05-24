use ratatui::{backend::Backend, layout::Rect, widgets::Paragraph, text::{Spans, Span}, style::{Style, Color, Modifier}, Frame};

/// Render the PrismX beam logo at the given area.
/// The logo is composed of unicode beam lines forming an X
/// with the label centered in the middle.
pub fn render_beam_logo<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let label = "PrismX";
    let width = label.len() as u16 + 6; // account for surrounding beams

    let cyan = Style::default().fg(Color::Cyan);
    let magenta = Style::default().fg(Color::Magenta);
    let white = Style::default().fg(Color::White).add_modifier(Modifier::BOLD);

    let top = Spans::from(Span::styled(format!("{:^width$}", "╱╲", width = width as usize), cyan));
    let middle = Spans::from(vec![
        Span::styled("╲━━", magenta),
        Span::styled(label, white),
        Span::styled("━━╱", magenta),
    ]);
    let bottom = Spans::from(Span::styled(format!("{:^width$}", "╲╱", width = width as usize), cyan));

    let x = area.x + area.width.saturating_sub(width) / 2;
    let logo_area = Rect::new(x, area.y, width, 3);
    let para = Paragraph::new(vec![top, middle, bottom]);
    f.render_widget(para, logo_area);
}


