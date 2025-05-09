use ratatui::{
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
    render_context_menu(f, area, state);
}

fn render_radial(f: &mut Frame<'_>, area: Rect, state: &MindmapState) {
    let center_x = area.x + area.width / 2;
    let center_y = area.y + area.height / 2;

    let mut labels = vec![];

    if let Some(root) = state.nodes.get(&state.root_id) {
        labels.push((center_x, center_y, &root.label, state.selected == Some(root.id)));

        let radius = 5.min(area.width / 2).min(area.height / 2);
        let child_count = root.children.len();

        for (i, child_id) in root.children.iter().enumerate() {
            if let Some(child) = state.nodes.get(child_id) {
                let angle = (i as f32 / child_count as f32) * std::f32::consts::TAU;
                let dx = (radius as f32 * angle.cos()) as u16;
                let dy = (radius as f32 * angle.sin()) as u16;
                let x = center_x.saturating_add_signed(dx as i16 - 5);
                let y = center_y.saturating_add_signed(dy as i16);
                labels.push((x, y, &child.label, state.selected == Some(child.id)));

                // Visual link label
                if let Some(parent_id) = child.parent {
                    if let Some(parent) = state.nodes.get(&parent_id) {
                        let label = format!("{} -> {}", parent.label, child.label);
                        let area = Rect::new(center_x.saturating_sub(10), y + 1, 20, 1);
                        let para = Paragraph::new(label).style(Style::default().fg(Color::DarkGray));
                        f.render_widget(para, area);
                    }
                }
            }
        }
    }

    for (x, y, label, selected) in labels {
        let block = Paragraph::new(label.to_string())
            .style(if selected {
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            });
        let w = label.len() as u16 + 2;
        let h = 1;
        let area = Rect::new(x.min(area.width.saturating_sub(w)), y.min(area.height - h), w, h);
        f.render_widget(block, area);
    }
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

fn render_context_menu(f: &mut Frame<'_>, area: Rect, state: &MindmapState) {
    if state.context_open {
        let items = vec![
            "Rename",
            "Add Tag",
            "Delete",
            "Convert to Project",
        ];
        let lines: Vec<Line> = items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let style = if i == state.context_selection {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                Line::from(Span::styled(*item, style))
            })
            .collect();

        let block = Block::default()
            .title("Node Menu")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));

        let para = Paragraph::new(lines).block(block);
        let width = 24;
        let height = items.len() as u16 + 2;
        let x = area.width.saturating_sub(width) - 1;
        let y = area.y + 1;

        f.render_widget(para, Rect::new(x, y, width, height));
    }
}