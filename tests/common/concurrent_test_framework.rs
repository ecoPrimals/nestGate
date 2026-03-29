//! Modern Concurrent Test Framework
//!
//! Replaces sleep()-based testing with event-driven, truly concurrent patterns.
//!
//! ## Philosophy
//!
//! **OLD**: Sleep-based testing is:
//! - Serial and slow
//! - Hides race conditions
//! - Fragile (timing-dependent)
//! - Anti-pattern for concurrent Rust
//!
//! **NEW**: Event-driven testing is:
//! - Concurrent and fast
//! - Exposes real timing issues
//! - Robust (logic-based, not time-based)
//! - Idiomatic modern Rust
//!
//! ## Usage Examples
//!
//! ### Replace Simple Sleep with Condition Wait
//!
//! ```rust,ignore
//! // ❌ OLD: Time-based, fragile
//! assert!(service.is_ready());
//!
//! // ✅ NEW: Event-based, robust
//! wait_for_condition(
//!     || service.is_ready(),
//!     Duration::from_secs(5),
//!     "service to be ready"
//! ).await?;
//! ```
//!
//! ### Replace Multiple Sleeps with Barrier
//!
//! ```rust,ignore
//! // ❌ OLD: Sleep to coordinate tasks
//!
//! // ✅ NEW: Barrier for coordination
//! let barrier = Arc::new(Barrier::new(2));
//! let results = concurrent_execute(vec![
//!     Box::pin(task1()),
//!     Box::pin(task2()),
//! ]).await?;
//! ```

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::sync::{Barrier, Mutex, Notify, RwLock};
use tokio::time::{Instant, timeout};

/// Error type for concurrent test operations
use std::fmt;

// Simplified error type - thiserror not needed in test code
#[derive(Debug)]
pub enum ConcurrentTestError {
    /// Timeout waiting for condition
    Timeout { duration: Duration, what: String },

    /// Task panicked
    TaskPanic(String),

    /// Channel closed unexpectedly
    ChannelClosed,

    /// Custom error
    Custom(String),
}

impl fmt::Display for ConcurrentTestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Timeout { duration, what } => {
                write!(f, "Timeout after {duration:?} waiting for: {what}")
            }
            Self::TaskPanic(msg) => write!(f, "Task panicked: {msg}"),
            Self::ChannelClosed => write!(f, "Channel closed unexpectedly"),
            Self::Custom(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for ConcurrentTestError {}

pub type Result<T> = std::result::Result<T, ConcurrentTestError>;

/// Wait for a condition to become true, polling efficiently
///
/// # Example
/// ```rust,ignore
/// wait_for_condition(
///     || service.is_ready(),
///     Duration::from_secs(5),
///     "service ready"
/// ).await?;
/// ```
pub async fn wait_for_condition<F>(
    mut condition: F,
    max_duration: Duration,
    what: &str,
) -> Result<()>
where
    F: FnMut() -> bool,
{
    let start = Instant::now();
    let poll_interval = Duration::from_millis(10); // Fast polling

    loop {
        if condition() {
            return Ok(());
        }

        if start.elapsed() > max_duration {
            return Err(ConcurrentTestError::Timeout {
                duration: max_duration,
                what: what.to_string(),
            });
        }

        tokio::time::sleep(poll_interval).await;
    }
}

/// Wait for an async condition to become true
///
/// # Example
/// ```rust,ignore
/// wait_for_async_condition(
///     || async { database.is_connected().await },
///     Duration::from_secs(5),
///     "database connection"
/// ).await?;
/// ```
pub async fn wait_for_async_condition<F, Fut>(
    mut condition: F,
    max_duration: Duration,
    what: &str,
) -> Result<()>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = bool>,
{
    let start = Instant::now();
    let poll_interval = Duration::from_millis(10);

    loop {
        if condition().await {
            return Ok(());
        }

        if start.elapsed() > max_duration {
            return Err(ConcurrentTestError::Timeout {
                duration: max_duration,
                what: what.to_string(),
            });
        }

        tokio::time::sleep(poll_interval).await;
    }
}

/// Execute multiple async tasks concurrently and collect results
///
/// # Example
/// ```rust,ignore
/// let results = concurrent_execute(vec![
///     Box::pin(task1()),
///     Box::pin(task2()),
///     Box::pin(task3()),
/// ]).await?;
/// ```
pub async fn concurrent_execute<T: Send + 'static>(
    tasks: Vec<Pin<Box<dyn Future<Output = T> + Send>>>,
) -> Result<Vec<T>> {
    let handles: Vec<_> = tasks.into_iter().map(|task| tokio::spawn(task)).collect();

    let mut results = Vec::new();
    for handle in handles {
        match handle.await {
            Ok(result) => results.push(result),
            Err(e) => {
                return Err(ConcurrentTestError::TaskPanic(format!("{:?}", e)));
            }
        }
    }

    Ok(results)
}

/// Execute tasks with a barrier for synchronization
///
/// All tasks will start at the same time after barrier
///
/// # Example
/// ```rust,ignore
/// let results = execute_with_barrier(vec![
///     Box::pin(async { expensive_operation1().await }),
///     Box::pin(async { expensive_operation2().await }),
/// ]).await?;
/// ```
pub async fn execute_with_barrier<T: Send + 'static>(
    tasks: Vec<Pin<Box<dyn Future<Output = T> + Send>>>,
) -> Result<Vec<T>> {
    let barrier = Arc::new(Barrier::new(tasks.len()));
    let handles: Vec<_> = tasks
        .into_iter()
        .map(|task| {
            let barrier = barrier.clone();
            tokio::spawn(async move {
                barrier.wait().await; // All tasks wait here
                task.await
            })
        })
        .collect();

    let mut results = Vec::new();
    for handle in handles {
        match handle.await {
            Ok(result) => results.push(result),
            Err(e) => {
                return Err(ConcurrentTestError::TaskPanic(format!("{:?}", e)));
            }
        }
    }

    Ok(results)
}

/// Wait for a notification with timeout
///
/// # Example
/// ```rust,ignore
/// let notify = Arc::new(Notify::new());
/// let notify_clone = notify.clone();
///
/// tokio::spawn(async move {
///     do_work().await;
///     notify_clone.notify_one();
/// });
///
/// wait_for_notify(notify, Duration::from_secs(5), "work completion").await?;
/// ```
pub async fn wait_for_notify(
    notify: Arc<Notify>,
    max_duration: Duration,
    what: &str,
) -> Result<()> {
    match timeout(max_duration, notify.notified()).await {
        Ok(_) => Ok(()),
        Err(_) => Err(ConcurrentTestError::Timeout {
            duration: max_duration,
            what: what.to_string(),
        }),
    }
}

/// Wait for multiple events using a channel
///
/// # Example
/// ```rust,ignore
/// let (tx, rx) = tokio::sync::mpsc::channel(10);
///
/// tokio::spawn(async move {
///     for i in 0..5 {
///         process_item(i).await;
///         tx.send(i).await.unwrap();
///     }
/// });
///
/// let events = wait_for_events(rx, 5, Duration::from_secs(10), "processing completion").await?;
/// assert_eq!(events.len(), 5);
/// ```
pub async fn wait_for_events<T>(
    mut receiver: tokio::sync::mpsc::Receiver<T>,
    expected_count: usize,
    max_duration: Duration,
    what: &str,
) -> Result<Vec<T>> {
    let start = Instant::now();
    let mut events = Vec::with_capacity(expected_count);

    while events.len() < expected_count {
        let remaining = max_duration.saturating_sub(start.elapsed());

        match timeout(remaining, receiver.recv()).await {
            Ok(Some(event)) => events.push(event),
            Ok(None) => return Err(ConcurrentTestError::ChannelClosed),
            Err(_) => {
                return Err(ConcurrentTestError::Timeout {
                    duration: max_duration,
                    what: format!("{} (got {}/{} events)", what, events.len(), expected_count),
                });
            }
        }
    }

    Ok(events)
}

/// Retry an operation with exponential backoff
///
/// # Example
/// ```rust,ignore
/// let result = retry_with_backoff(
///     || async { unreliable_service.call().await },
///     3, // max attempts
///     Duration::from_millis(100), // initial backoff
///     "service call"
/// ).await?;
/// ```
pub async fn retry_with_backoff<F, Fut, T, E>(
    mut operation: F,
    max_attempts: usize,
    initial_backoff: Duration,
    what: &str,
) -> std::result::Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = std::result::Result<T, E>>,
    E: std::fmt::Display,
{
    let mut backoff = initial_backoff;

    for attempt in 1..=max_attempts {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) if attempt == max_attempts => return Err(e),
            Err(e) => {
                eprintln!(
                    "Attempt {}/{} failed for {}: {}",
                    attempt, max_attempts, what, e
                );
                tokio::time::sleep(backoff).await;
                backoff *= 2; // Exponential backoff
            }
        }
    }

    unreachable!()
}

/// State machine helper for waiting on state transitions
///
/// # Example
/// ```rust,ignore
/// #[derive(Clone, PartialEq)]
/// enum ServiceState { Starting, Ready, Stopped }
///
/// let state = Arc::new(RwLock::new(ServiceState::Starting));
/// let state_clone = state.clone();
///
/// tokio::spawn(async move {
///     initialize_service().await;
///     *state_clone.write().await = ServiceState::Ready;
/// });
///
/// wait_for_state(state, ServiceState::Ready, Duration::from_secs(5), "service ready").await?;
/// ```
pub async fn wait_for_state<T>(
    state: Arc<RwLock<T>>,
    expected: T,
    max_duration: Duration,
    what: &str,
) -> Result<()>
where
    T: PartialEq + Clone,
{
    wait_for_async_condition(
        || async {
            let current = state.read().await;
            *current == expected
        },
        max_duration,
        what,
    )
    .await
}

/// Concurrent counter for coordinating multiple tasks
pub struct ConcurrentCounter {
    value: Arc<Mutex<usize>>,
    target: usize,
    notify: Arc<Notify>,
}

impl ConcurrentCounter {
    /// Create a new concurrent counter
    pub fn new(target: usize) -> Self {
        Self {
            value: Arc::new(Mutex::new(0)),
            target,
            notify: Arc::new(Notify::new()),
        }
    }

    /// Increment the counter
    pub async fn increment(&self) {
        let mut value = self.value.lock().await;
        *value += 1;
        if *value >= self.target {
            self.notify.notify_waiters();
        }
    }

    /// Wait for target to be reached
    pub async fn wait(&self, max_duration: Duration) -> Result<()> {
        // If the target was already reached before we subscribed to `notify`, the
        // corresponding `notify_waiters()` calls are lost — complete immediately.
        if *self.value.lock().await >= self.target {
            return Ok(());
        }
        wait_for_notify(self.notify.clone(), max_duration, "counter target").await
    }

    /// Get current value
    pub async fn get(&self) -> usize {
        *self.value.lock().await
    }
}

impl Clone for ConcurrentCounter {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            target: self.target,
            notify: self.notify.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_wait_for_condition() {
        // Event-driven: Set immediately in spawned task
        let ready = Arc::new(AtomicBool::new(false));
        let ready_clone = ready.clone();

        tokio::spawn(async move {
            // Set immediately - wait_for_condition will poll until ready
            ready_clone.store(true, Ordering::SeqCst);
        });

        // Wait for the condition to be set by spawned task
        let result = wait_for_condition(
            || ready.load(Ordering::SeqCst),
            Duration::from_secs(1),
            "test",
        )
        .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_concurrent_execute() {
        let tasks = vec![
            Box::pin(async { 1 }) as Pin<Box<dyn Future<Output = i32> + Send>>,
            Box::pin(async { 2 }),
            Box::pin(async { 3 }),
        ];

        let results = concurrent_execute(tasks).await.unwrap();
        assert_eq!(results.len(), 3);
        assert!(results.contains(&1));
        assert!(results.contains(&2));
        assert!(results.contains(&3));
    }

    #[tokio::test]
    async fn test_wait_for_notify() {
        let notify = Arc::new(Notify::new());
        let notify_clone = notify.clone();

        tokio::spawn(async move {
            // Notify immediately - wait_for_notify handles coordination
            notify_clone.notify_one();
        });

        let result = wait_for_notify(notify, Duration::from_secs(1), "test").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_concurrent_counter() {
        let counter = ConcurrentCounter::new(5);
        let handles: Vec<_> = (0..5)
            .map(|_| {
                let counter = counter.clone();
                tokio::spawn(async move {
                    // Increment immediately - tests true concurrency
                    counter.increment().await;
                })
            })
            .collect();

        for handle in handles {
            handle.await.unwrap();
        }

        let result = counter.wait(Duration::from_secs(1)).await;
        assert!(result.is_ok());
        assert_eq!(counter.get().await, 5);
    }
}
