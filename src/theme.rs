static mut CURRENT_THEME: &str = "dark";

pub fn toggle_theme() {
    unsafe {
        CURRENT_THEME = if CURRENT_THEME == "dark" { "light" } else { "dark" };
        println!("[THEME] Switched to: {}", CURRENT_THEME);
    }
}
