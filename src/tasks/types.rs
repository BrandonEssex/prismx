use tokio::task::JoinHandle;

/// Identifier for a spawned async task.
pub type TaskId = u64;

/// Handle to a running task, used for bookkeeping.
pub enum TaskHandle {
    /// Task running on the Tokio runtime.
    Tokio(JoinHandle<()>),
    /// Task executed via the fallback worker thread.
    Fallback,
}
