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
    
    map
}
