use super::core::{AppState, FavoriteEntry, LayoutSnapshot, DockLayout, ZenSyntax};
use crate::node::{Node, NodeID, NodeMap};
use crate::layout::{SIBLING_SPACING_X, CHILD_SPACING_Y, GEMX_HEADER_HEIGHT};
use crossterm::terminal;
use alloc::collections::HashSet;
use std::time::Instant;

impl AppState {
    pub fn get_selected_node(&self) -> Option<&Node> {
        self.selected.and_then(|id| self.nodes.get(&id))
    }

    pub fn get_selected_node_mut(&mut self) -> Option<&mut Node> {
        self.selected.and_then(|id| self.nodes.get_mut(&id))
    }

    pub fn set_selected(&mut self, id: Option<NodeID>) {
        self.selected = id;
        self.last_promoted_root = None;
    }

    pub fn dock_focus_prev(&mut self) {
        let len = self.favorite_entries().len();
        if len == 0 {
            self.dock_focus_index = None;
            return;
        }
        self.dock_focus_index = Some(match self.dock_focus_index.unwrap_or(0) {
            0 => len - 1,
            i => i - 1,
        });
        if let Some(idx) = self.dock_focus_index {
            if let Some(entry) = self.favorite_entries().get(idx) {
                self.status_message = entry.command.to_string();
                self.status_message_last_updated = Some(std::time::Instant::now());
            }
        }
    }

    pub fn dock_focus_next(&mut self) {
        let len = self.favorite_entries().len();
        if len == 0 {
            self.dock_focus_index = None;
            return;
        }
        self.dock_focus_index = Some(match self.dock_focus_index.unwrap_or(len - 1) {
            i if i + 1 >= len => 0,
            i => i + 1,
        });
        if let Some(idx) = self.dock_focus_index {
            if let Some(entry) = self.favorite_entries().get(idx) {
                self.status_message = entry.command.to_string();
                self.status_message_last_updated = Some(std::time::Instant::now());
            }
        }
    }

    pub fn visible_node_order(&self) -> Vec<NodeID> {
        let mut result = Vec::new();
        fn walk(id: NodeID, nodes: &NodeMap, out: &mut Vec<NodeID>) {
            out.push(id);
            if let Some(node) = nodes.get(&id) {
                if node.collapsed {
                    return;
                }
                for child in &node.children {
                    walk(*child, nodes, out);
                }
            }
        }
        for root_id in &self.root_nodes {
            if self.nodes.contains_key(root_id) {
                walk(*root_id, &self.nodes, &mut result);
            }
        }
        result
    }

    pub fn beam_color_for_mode(&self, mode: &str) -> crate::beam_color::BeamColor {
        match mode {
            "gemx" => self.gemx_beam_color,
            "zen" => self.zen_beam_color,
            "triage" => self.triage_beam_color,
            "settings" => self.settings_beam_color,
            _ => self.gemx_beam_color,
        }
    }

    pub fn cycle_beam_color(&mut self, mode: &str) {
        use crate::beam_color::BeamColor::*;
        let next = |c| match c {
            Prism => Infrared,
            Infrared => Aqua,
            Aqua => Emerald,
            Emerald => Ice,
            Ice => Prism,
        };
        match mode {
            "gemx" => self.gemx_beam_color = next(self.gemx_beam_color),
            "zen" => self.zen_beam_color = next(self.zen_beam_color),
            "triage" => self.triage_beam_color = next(self.triage_beam_color),
            "settings" => self.settings_beam_color = next(self.settings_beam_color),
            _ => {}
        }
    }

    pub fn cycle_beamx_panel_theme(&mut self) {
        use crate::beam_color::BeamColor::*;
        self.beamx_panel_theme = match self.beamx_panel_theme {
            Prism => Infrared,
            Infrared => Aqua,
            Aqua => Emerald,
            Emerald => Ice,
            Ice => Prism,
        };
    }

    pub fn beam_style_for_mode(&self, mode: &str) -> crate::beamx::BeamStyle {
        let mut style = crate::beamx::style_for_mode(mode);
        let (border, status, prism) = self.beam_color_for_mode(mode).palette();
        style.border_color = border;
        style.status_color = status;
        style.prism_color = prism;
        style
    }

    pub fn ensure_valid_roots(&mut self) {
        self.root_nodes.retain(|id| self.nodes.contains_key(id));
        if self.root_nodes.is_empty() && !self.nodes.is_empty() {
            if let Some((&first_id, _)) = self.nodes.iter().next() {
                if Some(first_id) != self.last_promoted_root {
                    self.root_nodes.push(first_id);
                    self.last_promoted_root = Some(first_id);
                    crate::log_debug!(
                        self,
                        "\u{26a0} root_nodes was empty — promoted Node {} to root",
                        first_id
                    );
                    tracing::debug!("[STATE] promoted node {} to root", first_id);
                }
            }
            if self.root_nodes.is_empty() {
                crate::log_warn!("fallback root promotion failed");
            }
        }
        self.root_nodes.sort_unstable();
        self.root_nodes.dedup();
    }

    pub fn audit_node_graph(&mut self) {
        use alloc::collections::VecDeque;
        for (&id, node) in &self.nodes {
            if node.label.trim().is_empty() {
                crate::log_debug!(self, "⚠ Node {} has no label", id);
            }
            if let Some(pid) = node.parent {
                if !self.nodes.contains_key(&pid) {
                    crate::log_debug!(self, "⚠ Node {} has missing parent {}", id, pid);
                } else if !self.nodes[&pid].children.contains(&id) {
                    crate::log_debug!(self, "⚠ Parent {} missing child link {}", pid, id);
                }
            } else if !self.root_nodes.contains(&id) {
                crate::log_debug!(self, "⚠ Node {} orphaned with no root", id);
            }
            let mut seen = HashSet::new();
            let mut current = node.parent;
            let mut depth = 0;
            while let Some(pid) = current {
                if pid == id {
                    crate::log_debug!(self, "♻ Cycle detected at node {}", id);
                    #[cfg(not(debug_assertions))]
                    panic!("❌ Node {} is its own ancestor", id);
                    break;
                }
                if !seen.insert(pid) {
                    break;
                }
                current = self.nodes.get(&pid).and_then(|n| n.parent);
                depth += 1;
                if depth > self.nodes.len() {
                    crate::log_debug!(self, "♻ Cycle detected by depth overflow at {}", id);
                    #[cfg(not(debug_assertions))]
                    panic!("❌ Cycle detected at node {}", id);
                    break;
                }
            }
        }
        let mut reachable = HashSet::new();
        let mut stack: VecDeque<NodeID> = self.root_nodes.iter().copied().collect();
        while let Some(id) = stack.pop_front() {
            if reachable.insert(id) {
                if let Some(n) = self.nodes.get(&id) {
                    for child in &n.children {
                        stack.push_back(*child);
                    }
                }
            }
        }
        for id in self.nodes.keys() {
            if !reachable.contains(id) {
                crate::log_debug!(self, "⚠ Node {} unreachable from roots", id);
            }
        }
        for (id, node) in self.nodes.iter_mut() {
            let before = node.children.len();
            node.children.sort_unstable();
            node.children.dedup();
            if node.children.len() != before {
                crate::log_debug!(self, "⚠ Node {} had duplicate children", id);
            }
        }
        self.root_nodes.sort_unstable();
        self.root_nodes.dedup();
    }

    pub fn audit_ancestry(&self) {
        for (&id, node) in &self.nodes {
            let mut current = node.parent;
            let mut depth = 0;
            while let Some(pid) = current {
                if pid == id {
                    tracing::error!("node {} is its own ancestor", id);
                    panic!("node {} is its own ancestor", id);
                }
                if depth > self.nodes.len() {
                    tracing::error!("cycle detected at node {}", id);
                    panic!("cycle detected at node {}", id);
                }
                current = self.nodes.get(&pid).and_then(|n| n.parent);
                depth += 1;
            }
        }
    }

    pub fn ensure_grid_positions(&mut self) {
        if self.auto_arrange {
            return;
        }
        let ids: Vec<NodeID> = self.nodes.keys().copied().collect();
        let (tw, th) = terminal::size().unwrap_or((80, 20));
        let mut used: HashSet<(i16, i16)> = self.nodes.values().map(|n| (n.x, n.y)).collect();
        let base_x = 6;
        let base_y = GEMX_HEADER_HEIGHT + 1;
        let step_x = SIBLING_SPACING_X * 2;
        let step_y = CHILD_SPACING_Y * 2;
        let max_y = th as i16 - 2;
        let max_x = tw as i16 - 4;
        for id in ids {
            if let Some(node) = self.nodes.get_mut(&id) {
                if node.x == 0 && node.y == 0 {
                    let mut x = base_x;
                    let mut y = base_y;
                    while used.contains(&(x, y)) {
                        y += step_y;
                        if y > max_y {
                            y = base_y;
                            x += step_x;
                            if x > max_x {
                                x = base_x;
                            }
                        }
                    }
                    node.x = x;
                    node.y = y;
                    used.insert((x, y));
                }
            }
        }
    }

    pub fn update_active_label(&mut self, c: char) {
        if let Some(node) = self.get_selected_node_mut() {
            node.label.push(c);
        }
    }

    pub fn delete_last_char(&mut self) {
        if let Some(node) = self.get_selected_node_mut() {
            node.label.pop();
        }
    }

    pub fn toggle_snap_grid(&mut self) {
        self.snap_to_grid = !self.snap_to_grid;
    }

    pub fn toggle_debug_border(&mut self) {
        self.debug_border = !self.debug_border;
    }

    pub fn syntax_from_extension(path: &str) -> ZenSyntax {
        let ext = std::path::Path::new(path)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        match ext.as_str() {
            "md" | "markdown" => ZenSyntax::Markdown,
            "rs" => ZenSyntax::Rust,
            "sh" | "bash" => ZenSyntax::Shell,
            "yml" | "yaml" => ZenSyntax::Yaml,
            "json" => ZenSyntax::Json,
            _ => ZenSyntax::None,
        }
    }

    pub fn zoom_in(&mut self) {
        self.zoom_scale = (self.zoom_scale + 0.1).min(1.5);
        self.zoom_locked_by_user = true;
        self.zoom_preview_last = Some(Instant::now());
        if let Some(id) = self.selected {
            crate::layout::zoom_to_anchor(self, id);
        }
    }

    pub fn zoom_out(&mut self) {
        self.zoom_scale = (self.zoom_scale - 0.1).max(0.5);
        self.zoom_locked_by_user = true;
        self.zoom_preview_last = Some(Instant::now());
        if let Some(id) = self.selected {
            crate::layout::zoom_to_anchor(self, id);
        }
    }

    pub fn zoom_reset(&mut self) {
        self.zoom_scale = 1.0;
        self.zoom_locked_by_user = false;
        self.zoom_preview_last = Some(Instant::now());
        if let Some(id) = self.selected {
            crate::layout::zoom_to_anchor(self, id);
        }
    }

    pub fn clear_fallback_promotions(&mut self) {
        self.fallback_promoted_this_session.clear();
        self.fallback_next_x = 6;
        self.fallback_next_y = GEMX_HEADER_HEIGHT + 2;
    }

    pub fn get_module_by_index(&self) -> &str {
        match self.module_switcher_index % 4 {
            0 => "gemx",
            1 => "zen",
            2 => "triage",
            3 => "settings",
            _ => "gemx",
        }
    }

    pub fn favorite_entries(&self) -> Vec<FavoriteEntry> {
        let default_favorites = [
            ("⚙️", "/settings"),
            ("📬", "/triage"),
            ("💭", "/gemx"),
            ("🧘", "/zen"),
            ("🔍", "/spotlight"),
        ];
        let limit = self.favorite_dock_limit.min(5);
        let mut all: Vec<FavoriteEntry> = self
            .plugin_favorites
            .iter()
            .cloned()
            .chain(
                default_favorites
                    .iter()
                    .map(|&(icon, cmd)| FavoriteEntry { icon, command: cmd }),
            )
            .take(limit)
            .collect();
        if self.mode == "gemx" && all.len() >= 3 {
            all[2].icon = "💬";
        }
        if (self.mode == "triage" || self.show_triage) && all.len() >= 2 {
            all[1].icon = "📫";
        }
        all
    }

    pub fn trigger_favorite(&mut self, index: usize) {
        let entries = self.favorite_entries();
        if let Some(entry) = entries.get(index) {
            self.spotlight_input = entry.command.to_string();
            self.show_spotlight = true;
            self.favorite_focus_index = Some(index);
            self.status_message = entry.command.to_string();
            self.status_message_last_updated = Some(std::time::Instant::now());
        }
    }
}

pub fn register_plugin_favorite(state: &mut AppState, icon: &'static str, command: &'static str) {
    if state.plugin_favorites.len() < 5 {
        state.plugin_favorites.push(FavoriteEntry { icon, command });
    }
}

impl AppState {
    pub fn save_layout_config(&self) {
        let layout = crate::state::serialize::capture(self);
        let mut cfg = crate::config::load_config();
        cfg.layout = Some(layout);
        crate::config::save_config(&cfg);
    }
}
