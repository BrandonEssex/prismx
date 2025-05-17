pub fn copy_node(text: &str) {
    println!("[CLIPBOARD] Copied: {}", text);
}

pub fn paste_node() {
    println!("[CLIPBOARD] Pasted: <not implemented>");
}

pub fn switch_workspace(name: &str) {
    println!("[WORKSPACE] Switched to: {}", name);
}
