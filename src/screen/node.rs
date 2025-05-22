pub link: Option<NodeID>,

pub fn new(id: NodeID, label: &str, parent: Option<NodeID>) -> Self {
    Self {
        id,
        label: label.into(),
        parent,
        children: vec![],
        collapsed: false,
        link: None,
    }
}
