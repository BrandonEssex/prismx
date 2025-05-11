use std::collections::HashMap;
use std::sync::RwLock;

use ratatui::{layout::Rect, Frame};

type PluginRenderFn = fn(&mut Frame<'_>, Rect);

lazy_static::lazy_static! {
    static ref SLOT_REGISTRY: RwLock<HashMap<String, PluginRenderFn>> = RwLock::new(HashMap::new());
}

pub struct PluginRegistry;

impl PluginRegistry {
    pub fn register_slot(name: &str, draw_fn: PluginRenderFn) {
        let mut registry = SLOT_REGISTRY.write().unwrap();
        registry.insert(name.to_string(), draw_fn);
    }

    pub fn draw_slot(name: &str, f: &mut Frame<'_>, area: Rect) {
        if let Some(draw_fn) = SLOT_REGISTRY.read().unwrap().get(name) {
            draw_fn(f, area);
        }
    }

    pub fn list_plugin_names() -> Vec<String> {
        SLOT_REGISTRY
            .read()
            .unwrap()
            .keys()
            .cloned()
            .collect()
    }

    pub fn clear_registry() {
        SLOT_REGISTRY.write().unwrap().clear();
    }
}