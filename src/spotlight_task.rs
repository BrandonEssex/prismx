use crossbeam_channel::{unbounded, Receiver, Sender};
use std::thread;

use crate::node::NodeMap;
use crate::spotlight::SpotlightResult;

pub struct SpotlightTask {
    pub result_rx: Receiver<Vec<SpotlightResult>>,
    cancel_tx: Sender<()>,
    handle: thread::JoinHandle<()>,
}

impl SpotlightTask {
    pub fn spawn(nodes: NodeMap, query: String) -> Self {
        let (result_tx, result_rx) = unbounded();
        let (cancel_tx, cancel_rx) = unbounded();
        let handle = thread::spawn(move || {
            let q = query.to_lowercase();
            let mut results = Vec::new();
            for (id, node) in nodes.iter() {
                if cancel_rx.try_recv().is_ok() {
                    return;
                }
                if node.label.to_lowercase().contains(&q) {
                    results.push(SpotlightResult::Node(*id));
                }
            }
            let _ = result_tx.send(results);
        });
        Self { result_rx, cancel_tx, handle }
    }

    pub fn cancel(self) {
        let _ = self.cancel_tx.send(());
        let _ = self.handle.join();
    }
}
