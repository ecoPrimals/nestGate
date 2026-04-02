//! Modern Synchronization Utilities for Tests
//!
//! This module provides proper synchronization primitives to replace `sleep()` calls in tests.
//! Philosophy: Test issues ARE production issues - fix root causes, not symptoms.

use anyhow::{Context, Result};
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Notify, RwLock};

/// Wait for a synchronous condition with timeout
///
/// # Example
/// ```no_run
/// wait_for_condition(|| service.is_ready(), Duration::from_secs(5)).await?;
/// ```
pub async fn wait_for_condition<F>(mut check: F, timeout: Duration) -> Result<()>
where
    F: FnMut() -> bool + Send,
{
    tokio::time::timeout(timeout, async {
        while !check() {
            tokio::task::yield_now().await;
        }
    })
    .await
    .context("Timeout waiting for condition")?;
    Ok(())
}

/// Wait for an async condition with timeout
///
/// # Example
/// ```no_run
/// wait_for_async(|| service.check_ready(), Duration::from_secs(5)).await?;
/// ```
pub async fn wait_for_async<F, Fut>(mut check: F, timeout: Duration) -> Result<()>
where
    F: FnMut() -> Fut + Send,
    Fut: Future<Output = bool> + Send,
{
    tokio::time::timeout(timeout, async {
        while !check().await {
            tokio::task::yield_now().await;
        }
    })
    .await
    .context("Timeout waiting for async condition")?;
    Ok(())
}

/// Wait for an async Result condition with timeout
///
/// # Example
/// ```no_run
/// wait_for_result(|| service.verify(), Duration::from_secs(5)).await?;
/// ```
pub async fn wait_for_result<F, Fut, T, E>(mut check: F, timeout: Duration) -> Result<T>
where
    F: FnMut() -> Fut + Send,
    Fut: Future<Output = Result<T, E>> + Send,
    E: std::fmt::Display,
{
    tokio::time::timeout(timeout, async {
        loop {
            match check().await {
                Ok(value) => return Ok(value),
                Err(_) => {
                    tokio::task::yield_now().await;
                }
            }
        }
    })
    .await
    .context("Timeout waiting for successful result")?
}

/// Service ready signaling - use this instead of sleep after starting services
///
/// # Example
/// ```no_run
/// let (signal, waiter) = ReadySignal::new();
///
/// // In service startup:
/// signal.signal();
///
/// // In test:
/// waiter.wait(Duration::from_secs(5)).await?;
/// ```
pub struct ReadySignal {
    notify: Arc<Notify>,
}

impl ReadySignal {
    pub fn new() -> (Self, ReadyWaiter) {
        let notify = Arc::new(Notify::new());
        (
            Self {
                notify: Arc::clone(&notify),
            },
            ReadyWaiter { notify },
        )
    }

    /// Signal that the service is ready
    pub fn signal(&self) {
        self.notify.notify_waiters();
    }

    /// Signal and keep signaling for future waiters
    pub fn signal_persistent(&self) {
        self.notify.notify_waiters();
    }
}

impl Default for ReadySignal {
    fn default() -> Self {
        Self::new().0
    }
}

pub struct ReadyWaiter {
    notify: Arc<Notify>,
}

impl ReadyWaiter {
    /// Wait for the ready signal with timeout
    pub async fn wait(&self, timeout: Duration) -> Result<()> {
        tokio::time::timeout(timeout, self.notify.notified())
            .await
            .context("Timeout waiting for ready signal")?;
        Ok(())
    }

    /// Wait for ready signal without timeout (use with caution!)
    pub async fn wait_indefinitely(&self) {
        self.notify.notified().await;
    }
}

/// Multi-stage completion tracker
///
/// Use this when you have multiple async tasks that need to complete
///
/// # Example
/// ```no_run
/// let tracker = CompletionTracker::new(3);
///
/// tokio::spawn(async move {
///     do_work().await;
///     tracker.mark_complete();
/// });
///
/// tracker.wait_all(Duration::from_secs(10)).await?;
/// ```
pub struct CompletionTracker {
    remaining: Arc<RwLock<usize>>,
    notify: Arc<Notify>,
}

impl CompletionTracker {
    /// Create a tracker for N tasks
    pub fn new(count: usize) -> Self {
        Self {
            remaining: Arc::new(RwLock::new(count)),
            notify: Arc::new(Notify::new()),
        }
    }

    /// Clone for passing to spawned tasks
    pub fn clone_handle(&self) -> CompletionHandle {
        CompletionHandle {
            remaining: Arc::clone(&self.remaining),
            notify: Arc::clone(&self.notify),
        }
    }

    /// Wait for all tasks to complete
    pub async fn wait_all(&self, timeout: Duration) -> Result<()> {
        tokio::time::timeout(timeout, async {
            loop {
                let remaining = *self.remaining.read().await;
                if remaining == 0 {
                    return;
                }
                self.notify.notified().await;
            }
        })
        .await
        .context("Timeout waiting for all tasks to complete")?;
        Ok(())
    }

    /// Check if all tasks are complete (non-blocking)
    pub async fn is_complete(&self) -> bool {
        *self.remaining.read().await == 0
    }
}

pub struct CompletionHandle {
    remaining: Arc<RwLock<usize>>,
    notify: Arc<Notify>,
}

impl CompletionHandle {
    /// Mark this task as complete
    pub async fn mark_complete(&self) {
        let mut remaining = self.remaining.write().await;
        if *remaining > 0 {
            *remaining -= 1;
            drop(remaining); // Release lock before notifying
            self.notify.notify_waiters();
        }
    }
}

/// Barrier for coordinating multiple tasks to start simultaneously
///
/// # Example
/// ```no_run
/// let barrier = Barrier::new(10);
///
/// for i in 0..10 {
///     let b = barrier.clone();
///     tokio::spawn(async move {
///         // Wait for all tasks to be ready
///         b.wait().await;
///         // Now all tasks start simultaneously
///         do_concurrent_work(i).await;
///     });
/// }
/// ```
pub struct Barrier {
    count: Arc<RwLock<usize>>,
    target: usize,
    notify: Arc<Notify>,
}

impl Barrier {
    /// Create a barrier for N tasks
    pub fn new(n: usize) -> Self {
        Self {
            count: Arc::new(RwLock::new(0)),
            target: n,
            notify: Arc::new(Notify::new()),
        }
    }

    /// Wait at the barrier
    pub async fn wait(&self) {
        {
            let mut count = self.count.write().await;
            *count += 1;
            if *count >= self.target {
                drop(count); // Release lock before notifying
                self.notify.notify_waiters();
                return;
            }
        }

        // Wait for barrier to open
        self.notify.notified().await;
    }
}

impl Clone for Barrier {
    fn clone(&self) -> Self {
        Self {
            count: Arc::clone(&self.count),
            target: self.target,
            notify: Arc::clone(&self.notify),
        }
    }
}

/// Poll an async function with exponential backoff
///
/// # Example
/// ```no_run
/// poll_with_backoff(
///     || async { check_service().await },
///     Duration::from_millis(10),
///     Duration::from_millis(1000),
///     Duration::from_secs(30),
/// ).await?;
/// ```
pub async fn poll_with_backoff<F, Fut>(
    mut check: F,
    initial_delay: Duration,
    max_delay: Duration,
    timeout: Duration,
) -> Result<()>
where
    F: FnMut() -> Fut + Send,
    Fut: Future<Output = bool> + Send,
{
    let start = std::time::Instant::now();
    let mut delay = initial_delay;

    tokio::time::timeout(timeout, async {
        loop {
            if check().await {
                return;
            }

            if start.elapsed() >= timeout {
                break;
            }

            tokio::time::sleep(delay).await;
            delay = std::cmp::min(delay * 2, max_delay);
        }
    })
    .await
    .context("Timeout in exponential backoff poll")?;

    Ok(())
}

/// Wait for multiple futures with a single timeout
///
/// # Example
/// ```no_run
/// let futures = vec![
///     async { task1().await },
///     async { task2().await },
///     async { task3().await },
/// ];
///
/// wait_all_with_timeout(futures, Duration::from_secs(10)).await?;
/// ```
pub async fn wait_all_with_timeout<F, Fut>(futures: Vec<Fut>, timeout: Duration) -> Result<Vec<F>>
where
    Fut: Future<Output = F> + Send,
{
    tokio::time::timeout(timeout, futures_util::future::join_all(futures))
        .await
        .context("Timeout waiting for all futures to complete")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

    #[tokio::test]
    async fn test_wait_for_condition() {
        let flag = Arc::new(AtomicBool::new(false));
        let flag_clone = Arc::clone(&flag);

        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(10)).await;
            flag_clone.store(true, Ordering::SeqCst);
        });

        wait_for_condition(|| flag.load(Ordering::SeqCst), Duration::from_secs(1))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_ready_signal() {
        let (signal, waiter) = ReadySignal::new();

        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(10)).await;
            signal.signal();
        });

        waiter.wait(Duration::from_secs(1)).await.unwrap();
    }

    #[tokio::test]
    async fn test_completion_tracker() {
        let tracker = CompletionTracker::new(3);
        let counter = Arc::new(AtomicUsize::new(0));

        for _ in 0..3 {
            let handle = tracker.clone_handle();
            let c = Arc::clone(&counter);
            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_millis(10)).await;
                c.fetch_add(1, Ordering::SeqCst);
                handle.mark_complete().await;
            });
        }

        tracker.wait_all(Duration::from_secs(1)).await.unwrap();
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_barrier() {
        let barrier = Barrier::new(5);
        let counter = Arc::new(AtomicUsize::new(0));

        let mut handles = vec![];
        for _ in 0..5 {
            let b = barrier.clone();
            let c = Arc::clone(&counter);
            handles.push(tokio::spawn(async move {
                b.wait().await;
                c.fetch_add(1, Ordering::SeqCst);
            }));
        }

        for handle in handles {
            handle.await.unwrap();
        }

        assert_eq!(counter.load(Ordering::SeqCst), 5);
    }
}
