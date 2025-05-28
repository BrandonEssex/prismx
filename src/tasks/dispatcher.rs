use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Mutex,
};

use once_cell::sync::Lazy;
use crossbeam_channel::{unbounded, Sender};
use tokio::runtime::{Handle, Runtime};

use crate::tasks::types::{TaskHandle, TaskId};

static TASK_COUNTER: AtomicU64 = AtomicU64::new(1);
static TASK_REGISTRY: Lazy<Mutex<HashMap<TaskId, (String, TaskHandle)>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

struct FallbackTask {
    id: TaskId,
    name: String,
    fut: Pin<Box<dyn Future<Output = ()> + Send + 'static>>,
}

static FALLBACK_SENDER: Lazy<Sender<FallbackTask>> = Lazy::new(|| {
    let (tx, rx) = unbounded::<FallbackTask>();
    std::thread::Builder::new()
        .name("task-worker".into())
        .spawn(move || {
            let rt = Runtime::new().expect("runtime");
            for task in rx {
                tracing::info!("[task] executing {}: {}", task.id, task.name);
                rt.block_on(task.fut);
                TASK_REGISTRY.lock().unwrap().remove(&task.id);
            }
        })
        .expect("spawn worker thread");
    tx
});

pub fn spawn_task<Fut>(name: impl Into<String>, fut: Fut) -> TaskId
where
    Fut: Future<Output = ()> + Send + 'static,
{
    let id = TASK_COUNTER.fetch_add(1, Ordering::SeqCst);
    let name_str = name.into();
    tracing::info!("[task] launch {}: {}", id, name_str);

    if Handle::try_current().is_ok() {
        let handle = tokio::spawn(fut);
        TASK_REGISTRY
            .lock()
            .unwrap()
            .insert(id, (name_str, TaskHandle::Tokio(handle)));
    } else {
        FALLBACK_SENDER
            .send(FallbackTask {
                id,
                name: name_str.clone(),
                fut: Box::pin(fut),
            })
            .expect("send to worker");
        TASK_REGISTRY
            .lock()
            .unwrap()
            .insert(id, (name_str, TaskHandle::Fallback));
    }

    id
}
