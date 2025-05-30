use ratatui::{prelude::*, widgets::Paragraph};
use ratatui::style::Color;
use crate::ui::animate::{shimmer, scale_color};

/// Draw a straight line between two points using simple box-drawing glyphs.
pub fn draw_line<B: Backend>(f: &mut Frame<B>, start: (i16, i16), end: (i16, i16)) {
    let (sx, sy) = start;
    let (ex, ey) = end;
    if sx == ex {
        let y0 = sy.min(ey);
        let y1 = sy.max(ey);
        for y in y0..=y1 {
            let rect = Rect::new(sx as u16, y as u16, 1, 1);
            f.render_widget(Paragraph::new("│"), rect);
        }
    } else if sy == ey {
        let x0 = sx.min(ex);
        let x1 = sx.max(ex);
        for x in x0..=x1 {
            let rect = Rect::new(x as u16, sy as u16, 1, 1);
            f.render_widget(Paragraph::new("─"), rect);
        }
    } else {
        // Draw an elbow using vertical then horizontal segment with proper corner
        draw_line(f, start, (sx, ey));
        let glyph = if sy < ey {
            if sx < ex { "┌" } else { "┐" }
        } else if sx < ex {
            "└"
        } else {
            "┘"
        };
        f.render_widget(Paragraph::new(glyph), Rect::new(sx as u16, ey as u16, 1, 1));
        draw_line(f, (sx, ey), end);
    }
}

/// Draw a line and place an arrow glyph at the end position.
pub fn draw_line_with_arrow<B: Backend>(
    f: &mut Frame<B>,
    start: (i16, i16),
    end: (i16, i16),
    arrow: &str,
) {
    draw_line(f, start, end);
    let (ex, ey) = end;
    if ex >= 0 && ey >= 0 {
        let rect = Rect::new(ex as u16, ey as u16, 1, 1);
        f.render_widget(Paragraph::new(arrow), rect);
    }
}

/// Draw a vertical line that fades from `color` using a shimmer effect.
pub fn draw_vertical_fade<B: Backend>(
    f: &mut Frame<B>,
    start: (i16, i16),
    end: (i16, i16),
    tick: u64,
    color: Color,
) {
    let (sx, sy) = start;
    let (ex, ey) = end;
    if sx != ex {
        return;
    }
    let y0 = sy.min(ey);
    let y1 = sy.max(ey);
    let len = (y1 - y0).max(1) as usize;
    for (i, y) in (y0..=y1).enumerate() {
        let ratio = 1.0 - i as f32 / len as f32;
        let c = scale_color(color, ratio);
        let style = shimmer(c, tick + i as u64);
        let rect = Rect::new(sx as u16, y as u16, 1, 1);
        f.render_widget(Paragraph::new("│").style(style), rect);
    }
}

/// Draw a horizontal line that shimmers across its length.
pub fn draw_horizontal_shimmer<B: Backend>(
    f: &mut Frame<B>,
    start: (i16, i16),
    end: (i16, i16),
    tick: u64,
    color: Color,
) {
    let (sx, sy) = start;
    let (ex, ey) = end;
    if sy != ey {
        return;
    }
    let x0 = sx.min(ex);
    let x1 = sx.max(ex);
    for (i, x) in (x0..=x1).enumerate() {
        let style = shimmer(color, tick + i as u64);
        let rect = Rect::new(x as u16, sy as u16, 1, 1);
        f.render_widget(Paragraph::new("─").style(style), rect);
    }
}

/// Draw a ghost line using shimmer style for previewing connections.
pub fn draw_ghost_line<B: Backend>(
    f: &mut Frame<B>,
    start: (i16, i16),
    end: (i16, i16),
    tick: u64,
    color: Color,
) {
    let (sx, sy) = start;
    let (ex, ey) = end;
    if sx == ex {
        let y0 = sy.min(ey);
        let y1 = sy.max(ey);
        for (i, y) in (y0..=y1).enumerate() {
            let style = shimmer(scale_color(color, 0.5), tick + i as u64);
            let rect = Rect::new(sx as u16, y as u16, 1, 1);
            f.render_widget(Paragraph::new("│").style(style), rect);
        }
    } else if sy == ey {
        let x0 = sx.min(ex);
        let x1 = sx.max(ex);
        for (i, x) in (x0..=x1).enumerate() {
            let style = shimmer(scale_color(color, 0.5), tick + i as u64);
            let rect = Rect::new(x as u16, sy as u16, 1, 1);
            f.render_widget(Paragraph::new("─").style(style), rect);
        }
    } else {
        draw_ghost_line(f, start, (sx, ey), tick, color);
        let glyph = if sy < ey {
            if sx < ex { "┌" } else { "┐" }
        } else if sx < ex {
            "└"
        } else {
            "┘"
        };
        let style = shimmer(scale_color(color, 0.5), tick);
        f.render_widget(Paragraph::new(glyph).style(style), Rect::new(sx as u16, ey as u16, 1, 1));
        draw_ghost_line(f, (sx, ey), end, tick, color);
    }
}

/// Draw a shimmering anchor trail between two points.
pub fn draw_anchor_trail<B: Backend>(
    f: &mut Frame<B>,
    start: (i16, i16),
    end: (i16, i16),
    tick: u64,
    color: Color,
) {
    draw_ghost_line(f, start, end, tick, color);
    let (ex, ey) = end;
    let style = shimmer(color, tick);
    if ex >= 0 && ey >= 0 {
        let rect = Rect::new(ex as u16, ey as u16, 1, 1);
        f.render_widget(Paragraph::new("●").style(style), rect);
    }
}
