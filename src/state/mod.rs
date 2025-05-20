use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

mod mindmap;
mod zen;
mod hotkeys;

pub use mindmap::*;
pub use zen::*;
pub use hotkeys::*;

pub struct AppState {
    pub mode: String,
    pub zen_buffer: Vec<String>,
    pub root: Rc<RefCell<Node>>,
    pub flat_nodes: Vec<(usize, Rc<RefCell<Node>>)>,
    pub active_node: usize,
    pub edit_mode: bool,
    pub edit_ready: bool,
    pub spotlight_input: String,
    pub show_spotlight: bool,
    pub show_triage: bool,
    pub show_keymap: bool,
    pub module_switcher_open: bool,
    pub module_switcher_index: usize,
    pub hotkeys: HashMap<String, String>,
}

impl Default for AppState {
    fn default() -> Self {
        let root = Rc::new(RefCell::new(Node::default_root()));
        let flat = flatten_nodes(&root);

        Self {
            mode: "mindmap".into(),
            zen_buffer: vec!["".into()],
            root,
            flat_nodes: flat,
            active_node: 0,
            edit_mode: false,
            edit_ready: false,
            spotlight_input: String::new(),
            show_spotlight: false,
            show_triage: false,
            show_keymap: false,
            module_switcher_open: false,
            module_switcher_index: 0,
            hotkeys: load_default_hotkeys(),
        }
    }
}
