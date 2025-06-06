use ratatui::{prelude::*, widgets::Paragraph};
use ratatui::style::{Color, Style};
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

/// Draw a straight line between two points using box-drawing glyphs with color.
pub fn draw_line_colored<B: Backend>(
    f: &mut Frame<B>,
    start: (i16, i16),
    end: (i16, i16),
    color: Color,
) {
    let style = Style::default().fg(color);
    let (sx, sy) = start;
    let (ex, ey) = end;
    if sx == ex {
        let y0 = sy.min(ey);
        let y1 = sy.max(ey);
        for y in y0..=y1 {
            if sx < 0 || y < 0 {
                continue;
            }
            let rect = Rect::new(sx as u16, y as u16, 1, 1);
            f.render_widget(Paragraph::new("│").style(style), rect);
        }
    } else if sy == ey {
        let x0 = sx.min(ex);
        let x1 = sx.max(ex);
        for x in x0..=x1 {
            if x < 0 || sy < 0 {
                continue;
            }
            let rect = Rect::new(x as u16, sy as u16, 1, 1);
            f.render_widget(Paragraph::new("─").style(style), rect);
        }
    } else {
        draw_line_colored(f, start, (sx, ey), color);
        let glyph = if sy < ey {
            if sx < ex { "┌" } else { "┐" }
        } else if sx < ex {
            "└"
        } else {
            "┘"
        };
        if sx >= 0 && ey >= 0 {
            f.render_widget(Paragraph::new(glyph).style(style), Rect::new(sx as u16, ey as u16, 1, 1));
        }
        draw_line_colored(f, (sx, ey), end, color);
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

/// Render an arrow glyph at the provided point using optional shimmer style.
pub fn draw_arrow<B: Backend>(
    f: &mut Frame<B>,
    pos: (i16, i16),
    tick: u64,
    color: Color,
    arrow: &str,
    shimmer_enabled: bool,
) {
    let (x, y) = pos;
    if x < 0 || y < 0 {
        return;
    }
    let style = if shimmer_enabled {
        shimmer(color, tick)
    } else {
        Style::default().fg(color)
    };
    let rect = Rect::new(x as u16, y as u16, 1, 1);
    f.render_widget(Paragraph::new(arrow).style(style), rect);
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

/// Draw a dashed line between two points.
/// Used when connectors span long distances or rendering fails.
pub fn draw_dashed_line<B: Backend>(
    f: &mut Frame<B>,
    start: (i16, i16),
    end: (i16, i16),
) {
    let (sx, sy) = start;
    let (ex, ey) = end;
    if sx == ex {
        let y0 = sy.min(ey);
        let y1 = sy.max(ey);
        let mut on = true;
        for y in y0..=y1 {
            if on {
                let rect = Rect::new(sx as u16, y as u16, 1, 1);
                f.render_widget(Paragraph::new("┊"), rect);
            }
            on = !on;
        }
    } else if sy == ey {
        let x0 = sx.min(ex);
        let x1 = sx.max(ex);
        let mut on = true;
        for x in x0..=x1 {
            if on {
                let rect = Rect::new(x as u16, sy as u16, 1, 1);
                f.render_widget(Paragraph::new("┄"), rect);
            }
            on = !on;
        }
    } else {
        draw_dashed_line(f, start, (sx, ey));
        let glyph = if sy < ey {
            if sx < ex { "┌" } else { "┐" }
        } else if sx < ex {
            "└"
        } else {
            "┘"
        };
        f.render_widget(Paragraph::new(glyph), Rect::new(sx as u16, ey as u16, 1, 1));
        draw_dashed_line(f, (sx, ey), end);
    }
}

/// Draw a short elbow connector when nodes are nearly overlapping.
pub fn draw_curved_short<B: Backend>(
    f: &mut Frame<B>,
    start: (i16, i16),
    end: (i16, i16),
) {
    let (sx, sy) = start;
    let (ex, ey) = end;
    // route to the right of the target point
    let mid_x = sx + 1;
    draw_line(f, start, (mid_x, sy));
    draw_line(f, (mid_x, sy), (mid_x, ey));
    draw_line(f, (mid_x, ey), (ex, ey));
}

/// Draw a connector from a parent point to a child point using an
/// L-shaped path through their midpoint.
pub fn draw_midpoint_connector<B: Backend>(
    f: &mut Frame<B>,
    start: (i16, i16),
    end: (i16, i16),
    color: Color,
) {
    let (sx, sy) = start;
    let (ex, ey) = end;

    let mid_y = (sy + ey) / 2;
    draw_line_colored(f, (sx, sy), (sx, mid_y), color);
    draw_line_colored(f, (sx, mid_y), (ex, mid_y), color);
    draw_line_colored(f, (ex, mid_y), (ex, ey), color);
}
