use ratatui::{prelude::*, widgets::{Paragraph, Wrap, Block, Borders}, text::{Line, Span}};
use ratatui::style::{Style, Modifier};
use regex::Regex;
use crate::node::{NodeID, NodeMap};
use crate::state::AppState;
use crate::layout::engine::{
    layout_vertical, validate_layout, LayoutStatus,
};
use super::layout::{clamp_child_spacing, enforce_viewport_bounds};
use crate::ui::lines::{
    draw_vertical_fade,
    draw_horizontal_shimmer,
    draw_ghost_line,
    draw_anchor_trail,
    draw_line,
    draw_dashed_line,
    draw_curved_short,
    draw_arrow,
};
use crate::theme::beam_color::{parent_line_color, sibling_line_color};
use crate::beam_color::BeamColor;
use crate::ui::beamx::{BeamXMode, BeamXStyle, InsertCursorKind, render_insert_cursor, trail_style, bright_color};
use crate::ui::animate::{scale_color, shimmer};
use crate::ui::overlay::{render_node_tooltip, render_layout_zones};
use std::collections::{HashSet, HashMap};
use crate::theme::layout::{node_max_width, NODE_WRAP_LABELS};
use crate::theme::icons::{ROOT_NODE, CHILD_NODE, SIBLING_NODE};
use crate::theme::characters::{DOWN_ARROW, RIGHT_ARROW};

const LONG_JUMP: i16 = 10;

fn compressed_label(label: &str, zoom: f32) -> String {
    if zoom > 0.5 {
        return label.to_string();
    }

    let mut base = label.split(|c| c == '#' || c == '@' || c == '[').next().unwrap_or(label).trim().to_string();
    if base.len() > 10 {
        base.truncate(10);
        base.push('…');
    }
    base
}

fn clamp_display_label(text: String, zoom: f32) -> (String, usize) {
    let limit = node_max_width(zoom);
    if text.chars().count() > limit {
        if NODE_WRAP_LABELS {
            let chars: Vec<char> = text.chars().collect();
            let first: String = chars[..limit].iter().collect();
            let rest: String = chars[limit..].iter().collect();
            let wrapped = format!("{}\n{}", first.trim_end(), rest);
            let width = wrapped.lines().map(|l| l.chars().count()).max().unwrap_or(0);
            return (wrapped, width);
        } else {
            let mut s: String = text.chars().take(limit.saturating_sub(1)).collect();
            s.push('…');
            let width = s.chars().count();
            return (s, width);
        }
    }
    let width = text.lines().map(|l| l.chars().count()).max().unwrap_or(0);
    (text, width)
}

fn styled_label_line(line: &str) -> Line<'static> {
    let re = Regex::new(r"^([^0-9]*?)(\d+)(.*)$").unwrap();
    if let Some(cap) = re.captures(line) {
        let prefix = cap.get(1).map(|m| m.as_str()).unwrap_or("");
        let number = cap.get(2).map(|m| m.as_str()).unwrap_or("");
        let rest = cap.get(3).map(|m| m.as_str()).unwrap_or("");
        let mut spans = Vec::new();
        if !prefix.is_empty() {
            spans.push(Span::styled(
                prefix.to_string(),
                Style::default().add_modifier(Modifier::ITALIC | Modifier::DIM),
            ));
        }
        if !number.is_empty() {
            spans.push(Span::styled(
                number.to_string(),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ));
        }
        if !rest.is_empty() {
            spans.push(Span::raw(rest.to_string()));
        }
        Line::from(spans)
    } else {
        Line::from(line.to_string())
    }
}

fn styled_lines(text: &str) -> Vec<Line<'static>> {
    text.lines().map(styled_label_line).collect()
}

/// Render a simple mindmap using the vertical layout engine.
pub fn render<B: Backend>(
    f: &mut Frame<B>,
    area: Rect,
    nodes: &mut NodeMap,
    roots: &[NodeID],
    state: &mut AppState,
    debug: bool,
) {
    let style = state.beam_style_for_mode("gemx");
    let tick = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        / 300) as u64;
    let title_style = shimmer(style.border_color, tick);
    let block = Block::default()
        .borders(Borders::NONE)
        .title(Span::styled("GemX Mindmap", title_style.add_modifier(Modifier::BOLD)))
        .title_alignment(Alignment::Center);
    f.render_widget(block, area);
    let spacing_y = clamp_child_spacing(state, roots, area.height as i16);
    let mut focus_set: HashSet<NodeID> = HashSet::new();
    if let Some(mut current) = state.selected {
        focus_set.insert(current);
        while let Some(parent) = nodes.get(&current).and_then(|n| n.parent) {
            focus_set.insert(parent);
            current = parent;
        }
    }

    let focus_opt = if focus_set.is_empty() { None } else { Some(&focus_set) };
    for &root in roots {
        layout_vertical(nodes, root, spacing_y, focus_opt);
    }

    let check = validate_layout(nodes, area);
    state.layout_status = check.status;

    enforce_viewport_bounds(nodes, area);

    if debug {
        render_layout_zones(f, area);
    }

    let mut problem_nodes: HashSet<NodeID> = HashSet::new();
    if debug {
        for id in check.offenders {
            problem_nodes.insert(id);
        }
    }

    let zoom = state.zoom_scale as f32;

    let tick = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        / 300) as u64;
    let theme = BeamColor::Prism;
    let p_color = parent_line_color(theme);
    let s_color = sibling_line_color(theme);
    let mut cursor_style = BeamXStyle::from(BeamXMode::Zen);
    cursor_style.border_color = p_color;
    cursor_style.status_color = s_color;

    // Determine scroll offset if content exceeds available height
    let max_y = nodes.values().map(|n| n.y).max().unwrap_or(0);
    let mut scroll = 0i16;
    if max_y >= area.height as i16 {
        scroll = max_y - area.height as i16 + 1;
    }

    let scale_x = |val: i16| -> i16 { area.x as i16 + ((val as f32) * zoom).round() as i16 };
    let scale_y = |val: i16| -> i16 { area.y as i16 + (((val - scroll) as f32) * zoom).round() as i16 };

    let highlight = state.highlight_focus_branch || state.debug_input_mode;
    let mut focus_nodes: HashSet<NodeID> = HashSet::new();
    let mut focus_pairs: HashSet<(NodeID, NodeID)> = HashSet::new();
    if highlight {
        if let Some(mut current) = state.selected {
            focus_nodes.insert(current);
            while let Some(parent) = nodes.get(&current).and_then(|n| n.parent) {
                focus_pairs.insert((parent, current));
                focus_nodes.insert(parent);
                current = parent;
            }
        }
    }
    let age = state.focus_age().unwrap_or(0);
    let fade = (age.min(600) as f32) / 600.0;
    let highlight_parent = scale_color(bright_color(p_color), fade);
    let highlight_sibling = scale_color(bright_color(s_color), fade);

    let mut display_map: HashMap<NodeID, (Vec<Line>, usize)> = HashMap::new();
    for node in nodes.values() {
        let mut text = if zoom <= 0.5 {
            compressed_label(&node.label, zoom)
        } else {
            node.label.clone()
        };
        let is_problem = problem_nodes.contains(&node.id);
        if state.hierarchy_icons {
            let icon = if node.parent.is_none() {
                ROOT_NODE
            } else {
                let parent_id = node.parent.unwrap();
                if nodes
                    .get(&parent_id)
                    .and_then(|p| p.children.first().copied())
                    == Some(node.id)
                {
                    CHILD_NODE
                } else {
                    SIBLING_NODE
                }
            };
            text = format!("{} {}", icon, text);
        }
        if debug && is_problem {
            text.push_str(" ⚠");
        }
        let (disp, width) = clamp_display_label(text, zoom);
        let lines = styled_lines(&disp);
        display_map.insert(node.id, (lines, width));
    }

    if debug || highlight {
        for node in nodes.values() {
            if node.children.is_empty() {
                continue;
            }

            let width_self = display_map.get(&node.id).map(|d| d.1 as i16).unwrap_or(0);
            let px = scale_x(node.x + width_self / 2);
            let beam_y = scale_y(node.y + (spacing_y - 1).max(1));
            let on_path = focus_nodes.contains(&node.id);

            // vertical beam from parent to sibling bar
            let parent_start = (px, scale_y(node.y + 1));
            let parent_end = (px, beam_y);
            let pcol = if on_path { highlight_parent } else { p_color };
            let pshimmer = state.beam_shimmer || on_path;
            if pshimmer {
                draw_vertical_fade(f, parent_start, parent_end, tick, pcol);
            } else {
                draw_line(f, parent_start, parent_end);
            }
            draw_arrow(
                f,
                parent_end,
                tick,
                pcol,
                DOWN_ARROW,
                pshimmer,
            );

            // collect child centers
            let mut child_centers = Vec::new();
            for cid in &node.children {
                if let Some(child) = nodes.get(cid) {
                    let w = display_map.get(cid).map(|d| d.1 as i16).unwrap_or(0);
                    child_centers.push((cid, scale_x(child.x + w / 2)));
                }
            }
            if child_centers.is_empty() {
                continue;
            }
            let min_x = child_centers.iter().map(|c| c.1).min().unwrap();
            let max_x = child_centers.iter().map(|c| c.1).max().unwrap();

            // horizontal connector across siblings
            let scol = if on_path { highlight_sibling } else { s_color };
            let sshimmer = state.beam_shimmer || on_path;
            if sshimmer {
                draw_horizontal_shimmer(
                    f,
                    (min_x, beam_y),
                    (max_x, beam_y),
                    tick,
                    scol,
                );
            } else {
                let hdist = (max_x - min_x).abs();
                if hdist > LONG_JUMP {
                    draw_dashed_line(f, (min_x, beam_y), (max_x, beam_y));
                } else {
                    draw_line(
                        f,
                        (min_x, beam_y),
                        (max_x, beam_y),
                    );
                }
            }

            // arrow across siblings
            let mid = ((min_x + max_x) / 2) as i16;
            draw_arrow(
                f,
                (mid, beam_y),
                tick,
                scol,
                RIGHT_ARROW,
                sshimmer,
            );

            // vertical drop to each child
            for (cid, cx) in child_centers {
                if let Some(child) = nodes.get(cid) {
                    let start = (cx, beam_y);
                    let end = (cx, scale_y(child.y));
                    let is_ghost = child.label.starts_with("node ");
                    let child_on_path = focus_pairs.contains(&(node.id, *cid));
                    let ccol = if child_on_path { highlight_parent } else { p_color };
                    let cshimmer = state.beam_shimmer || child_on_path;
                    if cshimmer {
                        if is_ghost {
                            draw_ghost_line(f, start, end, tick, ccol);
                        } else {
                            draw_vertical_fade(f, start, end, tick, ccol);
                        }
                    } else {
                        let vdist = (end.1 - start.1).abs();
                        if vdist <= 1 {
                            draw_curved_short(f, start, (end.0, end.1 + 1));
                        } else if vdist > LONG_JUMP {
                            draw_dashed_line(f, start, end);
                        } else {
                            draw_line(f, start, end);
                        }
                    }
                    draw_arrow(
                        f,
                        end,
                        tick,
                        ccol,
                        DOWN_ARROW,
                        cshimmer,
                    );
                }
            }
        }
    }

    // Ghost trails showing multiple inbound references
    if state.ghost_link_trails {
        let refs = state.inbound_links();
        for (target, sources) in refs {
            if sources.len() < 2 { continue; }
            for src in sources {
                if nodes.get(&target).and_then(|n| n.parent) == Some(src) { continue; }
                if let (Some(s), Some(t)) = (nodes.get(&src), nodes.get(&target)) {
                    let sa = display_map.get(&src).map(|d| d.1 as i16).unwrap_or(0);
                    let ta = display_map.get(&target).map(|d| d.1 as i16).unwrap_or(0);
                    let ax = scale_x(s.x + sa / 2);
                    let ay = scale_y(s.y);
                    let bx = scale_x(t.x + ta / 2);
                    let by = scale_y(t.y);
                    draw_ghost_line(f, (ax, ay), (bx, by), tick, p_color);
                }
            }
        }
    }

    // Preview reparent link if dragging over a potential parent
    if let (Some(src), Some(tgt)) = (state.dragging, state.drag_hover_target) {
        if let (Some(s), Some(t)) = (nodes.get(&src), nodes.get(&tgt)) {
            let sw = display_map.get(&src).map(|d| d.1 as i16).unwrap_or(0);
            let tw = display_map.get(&tgt).map(|d| d.1 as i16).unwrap_or(0);
            let ax = scale_x(s.x + sw / 2);
            let ay = scale_y(s.y);
            let bx = scale_x(t.x + tw / 2);
            let by = scale_y(t.y);
            draw_anchor_trail(f, (ax, ay), (bx, by), tick, p_color);
        }
    }

    // Draw nodes
    for node in nodes.values() {
        let x = scale_x(node.x);
        let y = scale_y(node.y);
        if x >= 0 && y >= 0 && x < area.right() as i16 && y < area.bottom() as i16 {
            let is_problem = problem_nodes.contains(&node.id);
            if let Some((display, width_chars)) = display_map.get(&node.id) {
                let width = ((*width_chars as f32) * zoom).ceil().max(1.0) as u16;
                let height = display.len() as u16;
                let rect = Rect::new(x as u16, y as u16, width, height.max(1));
                let mut para = Paragraph::new(display.clone());
                if NODE_WRAP_LABELS {
                    para = para.wrap(Wrap { trim: false });
                }
            if highlight && focus_nodes.contains(&node.id) {
                para = para.style(trail_style(highlight_parent, tick, fade));
            }
            if state.drag_hover_target == Some(node.id) {
                para = para.style(trail_style(highlight_sibling, tick, 1.0));
            }
                if debug && is_problem {
                    para = para.style(Style::default().fg(Color::LightRed));
                }
                f.render_widget(para, rect);

                let show_hover = state.drag_hover_target == Some(node.id);
                let show_focus = Some(node.id) == state.selected && age < 2000;
                if show_hover || show_focus {
                    render_node_tooltip(f, area, rect, &node.label);
                }

                if node.label.starts_with("node ") {
                    let kind = InsertCursorKind::Sibling;
                    let cx = rect.x + rect.width;
                    let cy = rect.y;
                    render_insert_cursor(f, (cx, cy), tick, kind, &cursor_style);
                }
            }
        }
    }
}
