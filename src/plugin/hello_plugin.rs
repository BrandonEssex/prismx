// FINAL FULL FILE DELIVERY
// Filename: src/plugins/hello_plugin.rs
// File Delivery Progress: 1/1 files delivered

use crate::plugin_api::{Plugin, PluginContext};

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn on_start(&mut self, ctx: &mut PluginContext) {
        ctx.log("✅ HelloPlugin started successfully.");
    }

    fn on_tick(&mut self, ctx: &mut PluginContext) {
        ctx.log("🔁 HelloPlugin ticking...");
    }

    fn render_dashboard(&self, ctx: &mut PluginContext) {
        ctx.draw_text("👋 Hello from the dashboard!", None);
    }
}
