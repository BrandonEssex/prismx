use crate::mindtrace::{MindTrace, TraceNode};

pub fn copy_node(trace: &MindTrace, id: &str) -> Option<String> {
    trace.nodes.get(id).map(|n| n.content.clone())
}

pub fn cut_node(trace: &mut MindTrace, id: &str) -> Option<String> {
    let content = trace.nodes.remove(id).map(|n| n.content);
    for node in trace.nodes.values_mut() {
        node.children.retain(|cid| cid != id);
    }
    content
}

pub fn paste_node(trace: &mut MindTrace, parent_id: &str, content: &str) {
    trace.add_node(parent_id, content);
}
