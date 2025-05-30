use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use crate::ui::drag::DragState;

/// Draggable sticky note widget used in the Triage panel.
#[derive(Clone, Debug)]
pub struct StickyNote {
    /// Short heading displayed at the top of the note.
    pub title: String,
    /// Multi-line text body for the note.
    pub body: String,
    /// Background color of the note.
    pub color: Color,
    /// Top-left X position in terminal cells.
    pub x: u16,
    /// Top-left Y position in terminal cells.
    pub y: u16,
    /// Whether this note is currently focused.
    pub focused: bool,
}

impl StickyNote {
    /// Create a new sticky note at the given coordinates.
    pub fn new<T: Into<String>, B: Into<String>>(title: T, body: B, color: Color, x: u16, y: u16) -> Self {
        Self {
            title: title.into(),
            body: body.into(),
            color,
            x,
            y,
            focused: false,
        }
    }

    /// Area occupied by this note.
    pub fn area(&self) -> Rect {
        Rect::new(self.x, self.y, 20, 8)
    }

    /// Move the note by the specified delta.
    pub fn translate(&mut self, dx: i16, dy: i16) {
        self.x = adjust(self.x, dx);
        self.y = adjust(self.y, dy);
    }

    /// Check if the provided coordinates are inside this note.
    pub fn contains(&self, x: u16, y: u16) -> bool {
        let area = self.area();
        x >= area.x && x < area.right() && y >= area.y && y < area.bottom()
    }

    /// Render the sticky note widget.
    pub fn render<B: Backend>(&self, f: &mut Frame<B>) {
        let area = self.area();
        let mut style = Style::default().bg(self.color);
        if self.focused {
            style = style.add_modifier(Modifier::BOLD);
        }
        let block = Block::default().borders(Borders::ALL).style(style);
        f.render_widget(block, area);

        let title = Paragraph::new(self.title.clone())
            .style(Style::default().add_modifier(Modifier::BOLD));
        f.render_widget(title, Rect::new(area.x + 1, area.y + 1, area.width - 2, 1));

        let body = Paragraph::new(self.body.clone()).wrap(Wrap { trim: true });
        f.render_widget(body, Rect::new(area.x + 1, area.y + 2, area.width - 2, area.height - 3));
    }
}

#[derive(Default, Debug)]
pub struct StickyPanel {
    pub notes: Vec<StickyNote>,
    pub visible: bool,
    pub drag: DragState,
    pub focus: Option<usize>,
}

impl StickyPanel {
    pub fn note_at(&self, x: u16, y: u16) -> Option<usize> {
        self.notes.iter().position(|n| n.contains(x, y))
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>) {
        if !self.visible { return; }
        for note in &self.notes {
            note.render(f);
        }
    }
}

fn adjust(val: u16, delta: i16) -> u16 {
    if delta.is_negative() {
        val.saturating_sub(delta.wrapping_abs() as u16)
    } else {
        val.saturating_add(delta as u16)
    }
}
