use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = prismx_write)]
    fn prismx_write(s: &str);
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MindmapNode {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub children: Vec<MindmapNode>,
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    let data = include_str!("../snapshots/mindmap.json");
    let root: MindmapNode = serde_json::from_str(data).expect("invalid mindmap");
    let mut out = String::new();
    render(&root, 0, &mut out);
    prismx_write(&out);
}

fn render(node: &MindmapNode, depth: usize, out: &mut String) {
    for _ in 0..depth {
        out.push_str("  ");
    }
    out.push_str("- ");
    out.push_str(&node.title);
    out.push('\n');
    for child in &node.children {
        render(child, depth + 1, out);
    }
}
