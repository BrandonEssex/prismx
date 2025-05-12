use std::path::Path;

pub fn route_dropped_file_to_plugin(path: &Path) -> Option<&'static str> {
    match path.extension().and_then(|e| e.to_str()) {
        Some("md") => Some("glow"),
        Some("json") => Some("plugin_doc_viewer"),
        Some("log") => Some("plugin_debug_console"),
        Some("txt") => Some("plugin_doc_viewer"),
        _ => None,
    }
}
