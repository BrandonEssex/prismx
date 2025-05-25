pub fn copy_node(text: &str) {
    tracing::debug!("[CLIPBOARD] Copied: {}", text);
}

pub fn paste_node() {
    tracing::debug!("[CLIPBOARD] Pasted: <not implemented>");
}

pub fn switch_workspace(name: &str) {
    tracing::debug!("[WORKSPACE] Switched to: {}", name);
}
