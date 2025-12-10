//! # Coordination Primitives for Concurrent Testing
//!
//! Event-driven coordination patterns that replace sleep-based timing.
//!
//! ## Anti-Pattern: Sleep-Based Coordination
//!
//! ```rust,ignore
//! // ❌ BAD: Hope timing works
//! start_service().await;
//! tokio::time::sleep(Duration::from_millis(100)).await; // Hope it's ready!
//! send_request().await;
//! ```
//!
//! ## Modern Pattern: Event-Driven
//!
//! ```rust,ignore
//! // ✅ GOOD: Explicit coordination
//! let signal = ReadySignal::new();
//! start_service(signal.clone()).await;
//! signal.wait_ready().await; // Wait for actual readiness
//! send_request().await; // Guaranteed ready!
//! ```

use std::sync::Arc;
use tokio::sync::{Notify, RwLock, oneshot, Barrier};

/// **Ready Signal** - Single readiness notification
///
/// Use when one task needs to signal readiness to another.
///
/// # Example
///
/// ```rust,ignore
/// let signal = ReadySignal::new();
///
/// tokio::spawn({
///     let signal = signal.clone();
///     async move {
///         setup_server().await;
///         signal.notify_ready().await; // Signal ready
///     }
/// });
///
/// signal.wait_ready().await; // Wait for signal
/// ```
#[derive(Clone)]
pub struct ReadySignal {
    notify: Arc<Notify>,
}

impl ReadySignal {
    /// Create a new ready signal
    pub fn new() -> Self {
        Self {
            notify: Arc::new(Notify::new()),
        }
    }

    /// Signal that the task is ready
    pub async fn notify_ready(&self) {
        self.notify.notify_one();
    }

    /// Wait for the ready signal
    pub async fn wait_ready(&self) {
        self.notify.notified().await;
    }
}

impl Default for ReadySignal {
    fn default() -> Self {
        Self::new()
    }
}

/// **Completion Barrier** - Wait for multiple tasks to complete
///
/// Use when you need to wait for N tasks to reach a synchronization point.
///
/// # Example
///
/// ```rust,ignore
/// let barrier = CompletionBarrier::new(3);
///
/// for i in 0..3 {
///     let barrier = barrier.clone();
///     tokio::spawn(async move {
///         do_work(i).await;
///         barrier.arrive().await; // Signal arrival
///     });
/// }
///
/// barrier.wait_all().await; // Wait for all 3
/// ```
#[derive(Clone)]
pub struct CompletionBarrier {
    barrier: Arc<Barrier>,
}

impl CompletionBarrier {
    /// Create a barrier for N tasks
    pub fn new(count: usize) -> Self {
        Self {
            barrier: Arc::new(Barrier::new(count)),
        }
    }

    /// Signal arrival at the barrier
    pub async fn arrive(&self) {
        self.barrier.wait().await;
    }

    /// Wait for all tasks to arrive (alias for arrive)
    pub async fn wait_all(&self) {
        self.arrive().await;
    }
}

/// **State Watcher** - Observe state changes
///
/// Use when multiple tasks need to observe state changes.
///
/// # Example
///
/// ```rust,ignore
/// let watcher = StateWatcher::new("initializing");
///
/// tokio::spawn({
///     let watcher = watcher.clone();
///     async move {
///         setup().await;
///         watcher.update("running").await;
///     }
/// });
///
/// watcher.wait_for("running").await; // Wait for specific state
/// ```
pub struct StateWatcher<T: Clone + PartialEq> {
    state: Arc<RwLock<T>>,
    notify: Arc<Notify>,
}

impl<T: Clone + PartialEq> StateWatcher<T> {
    /// Create a new state watcher with initial state
    pub fn new(initial: T) -> Self {
        Self {
            state: Arc::new(RwLock::new(initial)),
            notify: Arc::new(Notify::new()),
        }
    }

    /// Update the state and notify watchers
    pub async fn update(&self, new_state: T) {
        *self.state.write().await = new_state;
        self.notify.notify_waiters();
    }

    /// Get current state
    pub async fn current(&self) -> T {
        self.state.read().await.clone()
    }

    /// Wait for a specific state
    pub async fn wait_for(&self, target: T) {
        loop {
            {
                let current = self.state.read().await;
                if *current == target {
                    return;
                }
            }
            self.notify.notified().await;
        }
    }

    /// Wait for state to change from current value
    pub async fn wait_change(&self) {
        let current = self.current().await;
        loop {
            self.notify.notified().await;
            if self.current().await != current {
                return;
            }
        }
    }
}

impl<T: Clone + PartialEq> Clone for StateWatcher<T> {
    fn clone(&self) -> Self {
        Self {
            state: Arc::clone(&self.state),
            notify: Arc::clone(&self.notify),
        }
    }
}

/// **Oneshot Completion** - Single-use completion signal
///
/// Use when one task needs to signal completion to exactly one waiter.
///
/// # Example
///
/// ```rust,ignore
/// let (tx, rx) = oneshot_completion();
///
/// tokio::spawn(async move {
///     let result = compute().await;
///     tx.send(result).ok();
/// });
///
/// let result = rx.await.ok();
/// ```
pub fn oneshot_completion<T>() -> (oneshot::Sender<T>, oneshot::Receiver<T>) {
    oneshot::channel()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ready_signal() {
        let signal = ReadySignal::new();
        let signal_clone = signal.clone();

        tokio::spawn(async move {
            signal_clone.notify_ready().await;
        });

        signal.wait_ready().await; // Should not hang
    }

    #[tokio::test]
    async fn test_completion_barrier() {
        let barrier = CompletionBarrier::new(3);

        let handles: Vec<_> = (0..3)
            .map(|_| {
                let barrier = barrier.clone();
                tokio::spawn(async move {
                    barrier.arrive().await;
                })
            })
            .collect();

        for handle in handles {
            handle.await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_state_watcher() {
        let watcher = StateWatcher::new("init");
        let watcher_clone = watcher.clone();

        tokio::spawn(async move {
            watcher_clone.update("ready").await;
        });

        watcher.wait_for("ready").await;
        assert_eq!(watcher.current().await, "ready");
    }

    #[tokio::test]
    async fn test_oneshot_completion() {
        let (tx, rx) = oneshot_completion();

        tokio::spawn(async move {
            tx.send(42).ok();
        });

        let result = rx.await.unwrap();
        assert_eq!(result, 42);
    }
}

