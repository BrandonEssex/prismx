#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MindmapNode {
    pub id: String,
    pub title: String,
    pub children: Vec<MindmapNode>,
}

impl MindmapNode {
    pub fn new(id: &str, title: &str) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            children: vec![],
        }
    }

    pub fn add_child(&mut self, child: MindmapNode) {
        self.children.push(child);
    }
}
