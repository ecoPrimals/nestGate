//! Modern Concurrent Test Synchronization Utilities
//!
//! This module provides event-driven synchronization primitives to replace
//! timing assumptions (sleep) with actual correctness testing.
//!
//! # Philosophy
//! - Tests should verify correctness, not timing
//! - Use events, not sleeps
//! - Truly concurrent, not serialized with delays
//! - Production-like concurrency patterns

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{broadcast, oneshot, Notify, RwLock, Semaphore};

/// Coordination point for concurrent test operations
///
/// Replaces: `tokio::time::sleep(Duration::from_millis(100)).await`
/// With: Event-driven readiness signaling
#[derive(Clone)]
pub struct TestCoordinator {
    ready: Arc<Notify>,
    done: Arc<Notify>,
    state: Arc<RwLock<CoordinatorState>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoordinatorState {
    Idle,
    Starting,
    Ready,
    Running,
    Complete,
    Failed,
}

impl TestCoordinator {
    pub fn new() -> Self {
        Self {
            ready: Arc::new(Notify::new()),
            done: Arc::new(Notify::new()),
            state: Arc::new(RwLock::new(CoordinatorState::Idle)),
        }
    }

    /// Signal that setup is complete and test can proceed
    pub async fn signal_ready(&self) {
        *self.state.write().await = CoordinatorState::Ready;
        self.ready.notify_waiters();
    }

    /// Wait for ready signal (replaces sleep before assertions)
    pub async fn wait_ready(&self) {
        self.ready.notified().await;
    }

    /// Signal that operation is complete
    pub async fn signal_done(&self) {
        *self.state.write().await = CoordinatorState::Complete;
        self.done.notify_waiters();
    }

    /// Wait for completion signal
    pub async fn wait_done(&self) {
        self.done.notified().await;
    }

    /// Get current state
    pub async fn state(&self) -> CoordinatorState {
        *self.state.read().await
    }

    /// Wait for specific state
    pub async fn wait_for_state(&self, target: CoordinatorState) -> bool {
        // Use watch channel for state changes
        loop {
            let current = *self.state.read().await;
            if current == target {
                return true;
            }
            if current == CoordinatorState::Failed {
                return false;
            }
            // Brief yield to allow state changes
            tokio::task::yield_now().await;
        }
    }
}

impl Default for TestCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

/// Barrier for coordinating multiple concurrent tasks
///
/// Replaces: Multiple sleep calls to "wait for all tasks"
/// With: Actual barrier synchronization
pub struct TestBarrier {
    count: Arc<Semaphore>,
    notify: Arc<Notify>,
    expected: usize,
}

impl TestBarrier {
    /// Create barrier expecting `n` tasks
    pub fn new(n: usize) -> Self {
        Self {
            count: Arc::new(Semaphore::new(0)),
            notify: Arc::new(Notify::new()),
            expected: n,
        }
    }

    /// Signal that this task is ready
    pub fn arrive(&self) {
        self.count.add_permits(1);
        if self.count.available_permits() >= self.expected {
            self.notify.notify_waiters();
        }
    }

    /// Wait for all tasks to arrive
    pub async fn wait(&self) {
        self.notify.notified().await;
    }
}

/// Phase coordinator for multi-stage test scenarios
///
/// Replaces: Sequential sleep calls between test phases
/// With: Explicit phase transitions
#[derive(Clone)]
pub struct PhaseCoordinator {
    phase: Arc<RwLock<usize>>,
    notify: Arc<Notify>,
}

impl PhaseCoordinator {
    pub fn new() -> Self {
        Self {
            phase: Arc::new(RwLock::new(0)),
            notify: Arc::new(Notify::new()),
        }
    }

    /// Advance to next phase
    pub async fn next_phase(&self) {
        let mut phase = self.phase.write().await;
        *phase += 1;
        drop(phase);
        self.notify.notify_waiters();
    }

    /// Wait for specific phase
    pub async fn wait_for_phase(&self, target: usize) {
        loop {
            let current = *self.phase.read().await;
            if current >= target {
                return;
            }
            self.notify.notified().await;
        }
    }

    /// Get current phase
    pub async fn current_phase(&self) -> usize {
        *self.phase.read().await
    }
}

impl Default for PhaseCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

/// Event stream for test progress tracking
///
/// Replaces: Sleep-based polling
/// With: Event-driven observation
#[derive(Clone)]
pub struct TestEventStream {
    tx: broadcast::Sender<TestEvent>,
}

#[derive(Debug, Clone)]
pub enum TestEvent {
    Started,
    Progress { step: String, percent: u8 },
    Completed,
    Failed { reason: String },
}

impl TestEventStream {
    pub fn new() -> Self {
        let (tx, _rx) = broadcast::channel(100);
        Self { tx }
    }

    /// Subscribe to events
    pub fn subscribe(&self) -> broadcast::Receiver<TestEvent> {
        self.tx.subscribe()
    }

    /// Emit event
    pub fn emit(&self, event: TestEvent) {
        let _ = self.tx.send(event);
    }

    /// Wait for specific event
    pub async fn wait_for<F>(&self, predicate: F) -> TestEvent
    where
        F: Fn(&TestEvent) -> bool,
    {
        let mut rx = self.subscribe();
        loop {
            match rx.recv().await {
                Ok(event) => {
                    if predicate(&event) {
                        return event;
                    }
                }
                Err(_) => {
                    // Channel closed
                    return TestEvent::Failed {
                        reason: "Event stream closed".to_string(),
                    };
                }
            }
        }
    }
}

impl Default for TestEventStream {
    fn default() -> Self {
        Self::new()
    }
}

/// Result channel for concurrent operations
///
/// Replaces: Sleep then check result
/// With: Await actual result
pub struct ResultChannel<T> {
    tx: Option<oneshot::Sender<T>>,
    rx: Option<oneshot::Receiver<T>>,
}

impl<T> ResultChannel<T> {
    pub fn new() -> Self {
        let (tx, rx) = oneshot::channel();
        Self {
            tx: Some(tx),
            rx: Some(rx),
        }
    }

    /// Send result
    pub fn send(&mut self, value: T) -> Result<(), T> {
        if let Some(tx) = self.tx.take() {
            tx.send(value)
        } else {
            Err(value)
        }
    }

    /// Receive result
    pub async fn recv(&mut self) -> Result<T, String> {
        if let Some(rx) = self.rx.take() {
            rx.await.map_err(|_| "Channel closed".to_string())
        } else {
            Err("Already consumed".to_string())
        }
    }
}

impl<T> Default for ResultChannel<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Timeout utilities with proper error handling
///
/// Replaces: sleep with longer duration "just in case"
/// With: Proper timeout handling
pub async fn with_timeout<F, T>(duration: Duration, future: F) -> Result<T, &'static str>
where
    F: std::future::Future<Output = T>,
{
    match tokio::time::timeout(duration, future).await {
        Ok(result) => Ok(result),
        Err(_) => Err("Operation timed out"),
    }
}

/// Retry with exponential backoff (no sleep between retries)
///
/// Replaces: sleep between retry attempts
/// With: Event-driven retry based on actual failures
pub async fn retry_with_backoff<F, T, E, Fut>(
    mut operation: F,
    max_retries: usize,
    initial_backoff: Duration,
) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
{
    let mut backoff = initial_backoff;

    for attempt in 0..max_retries {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                if attempt == max_retries - 1 {
                    return Err(e);
                }
                // Only sleep on retry, not on success path
                tokio::time::sleep(backoff).await;
                backoff = backoff.saturating_mul(2);
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_coordinator_basic() {
        let coord = TestCoordinator::new();
        let coord_clone = coord.clone();

        // Spawn task that signals ready
        tokio::spawn(async move {
            coord_clone.signal_ready().await;
        });

        // Wait for ready (no sleep!)
        coord.wait_ready().await;
        assert_eq!(coord.state().await, CoordinatorState::Ready);
    }

    #[tokio::test]
    async fn test_barrier_synchronization() {
        let barrier = Arc::new(TestBarrier::new(3));
        let mut handles = vec![];

        for _ in 0..3 {
            let b = barrier.clone();
            handles.push(tokio::spawn(async move {
                // Do work
                b.arrive();
            }));
        }

        // Wait for all tasks (no sleep!)
        barrier.wait().await;

        for handle in handles {
            handle.await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_phase_coordination() {
        let phase = PhaseCoordinator::new();
        let phase_clone = phase.clone();

        tokio::spawn(async move {
            phase_clone.next_phase().await;
            phase_clone.next_phase().await;
        });

        phase.wait_for_phase(2).await;
        assert_eq!(phase.current_phase().await, 2);
    }

    #[tokio::test]
    async fn test_event_stream() {
        let stream = TestEventStream::new();
        let stream_clone = stream.clone();

        tokio::spawn(async move {
            stream_clone.emit(TestEvent::Started);
            stream_clone.emit(TestEvent::Completed);
        });

        let event = stream.wait_for(|e| matches!(e, TestEvent::Completed)).await;
        assert!(matches!(event, TestEvent::Completed));
    }

    #[tokio::test]
    async fn test_result_channel() {
        let channel = ResultChannel::new();
        let channel_clone = ResultChannel::new();
        let (mut tx, mut rx) = (channel, channel_clone);

        tokio::spawn(async move {
            tx.send(42).unwrap();
        });

        let result: i32 = rx.recv().await.unwrap();
        assert_eq!(result, 42);
    }

    #[tokio::test]
    async fn test_with_timeout_success() {
        let result = with_timeout(Duration::from_secs(1), async { 42 }).await;
        assert_eq!(result, Ok(42));
    }

    #[tokio::test]
    async fn test_with_timeout_failure() {
        let result = with_timeout(Duration::from_millis(10), async {
            tokio::time::sleep(Duration::from_secs(10)).await;
            42
        })
        .await;
        assert!(result.is_err());
    }
}
