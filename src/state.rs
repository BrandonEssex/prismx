pub fn add_child(&mut self) {
    let node = self.get_active_node();
    let child = Rc::new(RefCell::new(Node {
        label: "New Child".into(),
        children: vec![],
    }));
    node.borrow_mut().children.push(child);
    self.reflatten();
    self.active_node = self.flat_nodes.len() - 1;
    self.edit_mode = true;
}

pub fn add_sibling(&mut self) {
    if self.active_node == 0 {
        return;
    }

    let (target_depth, _) = self.flat_nodes[self.active_node];
    let mut parent_to_update: Option<Rc<RefCell<Node>>> = None;

    for (depth, node) in self.flat_nodes.iter().rev() {
        if *depth == target_depth - 1 {
            parent_to_update = Some(Rc::clone(node));
            break;
        }
    }

    if let Some(parent) = parent_to_update {
        parent.borrow_mut().children.push(Rc::new(RefCell::new(Node {
            label: "New Sibling".into(),
            children: vec![],
        })));
        self.reflatten();
        self.active_node = self.flat_nodes.len() - 1;
        self.edit_mode = true;
    }
}
