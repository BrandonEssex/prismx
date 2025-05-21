use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Node {
    pub label: String,
    pub children: Vec<Rc<RefCell<Node>>>,
    pub collapsed: bool,
}

impl Node {
    pub fn default_root() -> Self {
        Self {
            label: "Root".into(),
            collapsed: false,
            children: vec![
                Rc::new(RefCell::new(Self {
                    label: "Node A".into(),
                    collapsed: false,
                    children: vec![],
                })),
                Rc::new(RefCell::new(Self {
                    label: "Node B".into(),
                    collapsed: false,
                    children: vec![],
                })),
            ],
        }
    }
}

pub fn flatten_nodes(node: &Rc<RefCell<Node>>) -> Vec<(usize, Rc<RefCell<Node>>)> {
    fn recurse(
        node: &Rc<RefCell<Node>>,
        depth: usize,
        out: &mut Vec<(usize, Rc<RefCell<Node>>)>
    ) {
        out.push((depth, Rc::clone(node)));

        // Safely borrow once
        let children;
        let collapsed;
        {
            let n = node.borrow();
            collapsed = n.collapsed;
            children = n.children.clone(); // clone Vec<Rc<...>> — cheap
        }

        if collapsed {
            return;
        }

        for child in children {
            recurse(&child, depth + 1, out);
        }
    }

    let mut result = Vec::new();
    recurse(node, 0, &mut result);
    result
}

