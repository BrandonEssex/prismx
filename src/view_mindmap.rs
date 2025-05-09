use ratatui::{
    backend::Backend,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    text::{Span, Line},
    style::{Style, Modifier, Color},
    Frame,
};

use crate::mindmap_state::{MindmapState, MindmapLayout};

pub fn render_mindmap(f: &mut Frame<'_>, area: Rect, state: &MindmapState) {
    match state.layout {
        MindmapLayout::Radial => render_radial(f, area, state),
        MindmapLayout::Tree => render_tree(f, area, state),
        MindmapLayout::Timeline => render_timeline(f, area, state),
    }

    render_edit_overlay(f, area, state);
    render_hint_bar(f, area, &state.layout);
}

fn render_radial(f: &mut Frame<'_>, area: Rect, state: &MindmapState) {
    let mut lines = vec![];
    for node in state.nodes.values() {
        let selected = state.selected == Some(node.id);
        let mut label = node.label.clone();
        if selected {
            label = format!("> {}", label);
        }
        lines.push(Line::from(vec![Span::styled(
            label,
            if selected {
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            },
        )]));
    }

    let block = Block::default().title("Mindmap (Radial)").borders(Borders::ALL);
    let para = Paragraph::new(lines).block(block);
    f.render_widget(para, area);
}

fn render_tree(_f: &mut Frame<'_>, _area: Rect, _state: &MindmapState) {}
fn render_timeline(_f: &mut Frame<'_>, _area: Rect, _state: &MindmapState) {}

fn render_edit_overlay(f: &mut Frame<'_>, area: Rect, state: &MindmapState) {
    if let Some(_) = state.editing {
        let block = Block::default().title("Editing Node").borders(Borders::ALL);
        let para = Paragraph::new(Line::from(vec![Span::raw(state.edit_buffer.clone())])).block(block);
        f.render_widget(para, area);
    }
}

fn render_hint_bar(f: &mut Frame<'_>, area: Rect, layout: &MindmapLayout) {
    let hint = match layout {
        MindmapLayout::Radial => "Layout: Radial [m] switch",
        MindmapLayout::Tree => "Layout: Tree [m] switch",
        MindmapLayout::Timeline => "Layout: Timeline [m] switch",
    };

    let para = Paragraph::new(Line::from(vec![Span::styled(
        hint,
        Style::default().fg(Color::Gray),
    )]));

    let hint_area = Rect {
        x: area.x,
        y: area.y + area.height.saturating_sub(1),
        width: area.width,
        height: 1,
    };
    f.render_widget(para, hint_area);
}