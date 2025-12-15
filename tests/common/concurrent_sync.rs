// Concurrent test utilities - Modern robust patterns
// NO SLEEPS - Use proper synchronization

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Barrier, Notify, RwLock, Semaphore};
use tokio::time::timeout;

/// Concurrent test coordinator - replaces sleep-based coordination
pub struct TestCoordinator {
    barrier: Arc<Barrier>,
    notify: Arc<Notify>,
    completion_count: Arc<RwLock<usize>>,
}

impl TestCoordinator {
    /// Create coordinator for N concurrent tasks
    pub fn new(task_count: usize) -> Self {
        Self {
            barrier: Arc::new(Barrier::new(task_count)),
            notify: Arc::new(Notify::new()),
            completion_count: Arc::new(RwLock::new(0)),
        }
    }

    /// Wait for all tasks to reach this point
    pub async fn sync_point(&self) {
        self.barrier.wait().await;
    }

    /// Signal completion of a task
    pub async fn mark_complete(&self) {
        let mut count = self.completion_count.write().await;
        *count += 1;
        self.notify.notify_waiters();
    }

    /// Wait for N tasks to complete
    pub async fn wait_for_completions(
        &self,
        expected: usize,
        max_duration: Duration,
    ) -> Result<(), &'static str> {
        timeout(max_duration, async {
            loop {
                let count = *self.completion_count.read().await;
                if count >= expected {
                    return Ok(());
                }
                self.notify.notified().await;
            }
        })
        .await
        .map_err(|_| "Timeout waiting for completions")?
    }

    /// Clone for sharing across tasks
    pub fn clone_handle(&self) -> TestCoordinator {
        Self {
            barrier: Arc::clone(&self.barrier),
            notify: Arc::clone(&self.notify),
            completion_count: Arc::clone(&self.completion_count),
        }
    }
}

/// Rate limiter for concurrent operations - NO SLEEPS
pub struct ConcurrentRateLimiter {
    semaphore: Arc<Semaphore>,
}

impl ConcurrentRateLimiter {
    /// Create with max concurrent operations
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
        }
    }

    /// Acquire permission - blocks if at limit
    pub async fn acquire(&self) -> tokio::sync::SemaphorePermit<'_> {
        self.semaphore.acquire().await.expect("Semaphore closed")
    }

    /// Try acquire - returns None if at limit
    pub fn try_acquire(&self) -> Option<tokio::sync::SemaphorePermit<'_>> {
        self.semaphore.try_acquire().ok()
    }
}

/// Event-based test synchronization - replaces timing-based tests
pub struct EventSync {
    events: Arc<RwLock<Vec<String>>>,
    notify: Arc<Notify>,
}

impl EventSync {
    pub fn new() -> Self {
        Self {
            events: Arc::new(RwLock::new(Vec::new())),
            notify: Arc::new(Notify::new()),
        }
    }

    /// Record an event
    pub async fn record(&self, event: impl Into<String>) {
        self.events.write().await.push(event.into());
        self.notify.notify_waiters();
    }

    /// Wait for specific event
    pub async fn wait_for_event(
        &self,
        event: &str,
        max_duration: Duration,
    ) -> Result<(), &'static str> {
        timeout(max_duration, async {
            loop {
                let events = self.events.read().await;
                if events.iter().any(|e| e == event) {
                    return Ok(());
                }
                drop(events); // Release lock before waiting
                self.notify.notified().await;
            }
        })
        .await
        .map_err(|_| "Timeout waiting for event")?
    }

    /// Wait for event sequence
    pub async fn wait_for_sequence(
        &self,
        sequence: &[&str],
        max_duration: Duration,
    ) -> Result<(), &'static str> {
        timeout(max_duration, async {
            loop {
                let events = self.events.read().await;
                if events.len() >= sequence.len() {
                    let matches = events
                        .windows(sequence.len())
                        .any(|window| window.iter().zip(sequence).all(|(e, s)| e == s));
                    if matches {
                        return Ok(());
                    }
                }
                drop(events);
                self.notify.notified().await;
            }
        })
        .await
        .map_err(|_| "Timeout waiting for sequence")?
    }

    /// Get all recorded events
    pub async fn get_events(&self) -> Vec<String> {
        self.events.read().await.clone()
    }

    pub fn clone_handle(&self) -> Self {
        Self {
            events: Arc::clone(&self.events),
            notify: Arc::clone(&self.notify),
        }
    }
}

impl Default for EventSync {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Duration;

    #[tokio::test]
    async fn test_coordinator_sync_point() {
        let coord = TestCoordinator::new(3);
        let mut handles = vec![];

        for i in 0..3 {
            let c = coord.clone_handle();
            handles.push(tokio::spawn(async move {
                // All tasks reach this point concurrently
                c.sync_point().await;
                i
            }));
        }

        let results: Vec<_> = futures::future::join_all(handles)
            .await
            .into_iter()
            .map(|r| r.unwrap())
            .collect();

        assert_eq!(results.len(), 3);
    }

    #[tokio::test]
    async fn test_event_sync() {
        let sync = EventSync::new();
        let s = sync.clone_handle();

        // Spawn task that records event immediately (no artificial delay)
        tokio::spawn(async move {
            // No sleep! Test pure event-driven coordination
            s.record("event1").await;
        });

        // Wait for event - NO SLEEPS, event-driven
        sync.wait_for_event("event1", Duration::from_secs(1))
            .await
            .expect("Event should arrive immediately");
    }

    #[tokio::test]
    async fn test_rate_limiter() {
        let limiter = ConcurrentRateLimiter::new(2);

        // Acquire 2 permits
        let _permit1 = limiter.acquire().await;
        let _permit2 = limiter.acquire().await;

        // Third should fail to try_acquire
        assert!(limiter.try_acquire().is_none());

        // After dropping one, should succeed
        drop(_permit1);
        assert!(limiter.try_acquire().is_some());
    }
}
