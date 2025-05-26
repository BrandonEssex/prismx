use std::collections::HashMap;


pub fn load_default_hotkeys() -> HashMap<String, String> {
    let mut map = HashMap::new();

    map.insert("create_child".into(), "tab".into());
    map.insert("create_sibling".into(), "enter".into());
    map.insert("delete".into(), "ctrl-d".into());
    map.insert("drill_down".into(), "ctrl-w".into());
    map.insert("pop_up".into(), "ctrl-q".into());
    map.insert("create_free_node".into(), "ctrl-n".into());
    map.insert("toggle_collapsed".into(), "ctrl-t".into());
    map.insert("toggle_triage".into(), "ctrl-y".into()); // changed to avoid conflict
    map.insert("save".into(), "ctrl-x".into());
    map.insert("quit".into(), "ctrl-c".into());
    map.insert("help".into(), "?".into());
    map.insert("switch_module".into(), "shift-tab".into());
    map.insert("toggle_edit".into(), "ctrl-e".into());
    map.insert("toggle_keymap".into(), "ctrl-h".into());
    map.insert("mode_zen".into(), "ctrl-r".into());
    map.insert("undo".into(), "ctrl+z".into()); 
    map.insert("add_free_node".into(), "ctrl-n".into());
    map.insert("open_module_switcher".into(), "ctrl-space" .into());
    map.insert("start_drag".into(), "ctrl-r".into());
    map.insert("start_link".into(), "ctrl-l".into());
    map.insert("toggle_link_mode".into(), "ctrl-b".into());
    map.insert("redo".into(), "ctrl-shift-z".into());
    map.insert("toggle_snap_grid".into(), "ctrl-g".into());
    map.insert("zen_toggle_theme".into(), "ctrl-a".into());


    map
}

pub fn load_hotkeys() -> HashMap<String, String> {
    let mut map = load_default_hotkeys();
    if let Ok(s) = std::fs::read_to_string("config/shortcuts.toml") {
        if let Ok(user_map) = toml::from_str::<HashMap<String, String>>(&s) {
            map.extend(user_map);
        }
    }
    map
}
