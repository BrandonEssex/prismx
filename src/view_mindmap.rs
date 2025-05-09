use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    text::{Line, Span},
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
}

fn render_radial(f: &mut Frame<'_>, area: Rect, state: &MindmapState) {
    let center_x = area.width / 2;
    let center_y = area.height / 2;
    let radius = (area.width.min(area.height) / 3).max(6);

    let mut used = std::collections::HashSet::new();
    let mut labels = vec![];

    if let Some(root) = state.nodes.get(&state.root_id) {
        labels.push((center_x, center_y, &root.label, state.selected == Some(root.id)));

        let child_count = root.children.len().max(1);

        for (i, child_id) in root.children.iter().enumerate() {
            if let Some(child) = state.nodes.get(child_id) {
                let angle = (i as f32 / child_count as f32) * std::f32::consts::TAU;
                let dx = (radius as f32 * angle.cos()).round() as i16;
                let dy = (radius as f32 * angle.sin()).round() as i16;

                let x = center_x.saturating_add_signed(dx).saturating_sub(6);
                let y = center_y.saturating_add_signed(dy);
                if !used.insert((x, y)) { continue; }

                labels.push((x, y, &child.label, state.selected == Some(child.id)));
            }
        }
    }

    for (x, y, label, selected) in labels {
        let text = label.to_string();
        let w = text.len() as u16 + 2;
        let h = 1;
        let box_area = Rect::new(x.min(area.width - w), y.min(area.height - h), w, h);

        let para = Paragraph::new(text).style(
            if selected {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            },
        );
        f.render_widget(para, box_area);
    }
}

fn render_tree(_f: &mut Frame<'_>, _area: Rect, _state: &MindmapState) {}
fn render_timeline(_f: &mut Frame<'_>, _area: Rect, _state: &MindmapState) {}

fn render_edit_overlay(f: &mut Frame<'_>, area: Rect, state: &MindmapState) {
    if let Some(id) = state.editing {
        if let Some(_node) = state.nodes.get(&id) {
            let label = format!("✏ {}", state.edit_buffer);
            let para = Paragraph::new(Line::from(Span::styled(
                label,
                Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
            )))
            .block(Block::default().borders(Borders::ALL).title("Node Edit"));

            let center_x = area.width / 2;
            let center_y = area.height / 2;
            let w = label.len() as u16 + 4;
            let h = 3;
            let x = center_x.saturating_sub(w / 2);
            let y = center_y;

            f.render_widget(para, Rect::new(x, y, w, h));
        }
    }
}