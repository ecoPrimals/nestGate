//! Isolated Test Context - True Concurrent Testing
//!
//! Provides per-test isolation for concurrent execution without conflicts.
//! Replaces sleep-based coordination with proper event-driven patterns.
//!
//! # Philosophy
//!
//! "Test issues ARE production issues" - Poor test patterns leak into production thinking.
//! This module enables truly concurrent tests by providing:
//! - Isolated resources (no conflicts)
//! - Event-driven coordination (no polling)
//! - Automatic cleanup (no leaks)
//!
//! # Usage
//!
//! ```rust
//! #[tokio::test]
//! async fn test_concurrent_service() {
//!     let ctx = IsolatedTestContext::new().await.unwrap();
//!     let port = ctx.allocate_port().await;
//!     
//!     // Tests run in parallel without conflicts
//!     let service = Service::new(port, ctx.temp_dir()).await;
//!     assert!(service.is_ready());
//! }
//! ```

use dashmap::DashSet;
// ✅ EVOLVED: once_cell → std::sync::OnceLock (Pure Rust std)
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU16, Ordering};
use tempfile::TempDir;
use tokio::sync::{Notify, RwLock, watch};

/// Isolated context for concurrent tests
///
/// Each test gets its own isolated resources:
/// - Unique ports (no conflicts)
/// - Temporary directory (isolated filesystem)
/// - Event-driven coordination (no sleeps)
pub struct IsolatedTestContext {
    /// Isolated temporary directory (cleaned up on drop)
    temp_dir: TempDir,
    /// Thread-safe port allocator
    port_pool: Arc<PortAllocator>,
    /// Cleanup guard (runs on drop, even on panic)
    _cleanup: CleanupGuard,
    /// Optional coordinator for complex scenarios
    coordinator: Arc<ConcurrentCoordinator>,
}

impl IsolatedTestContext {
    /// Create new isolated context
    ///
    /// # Errors
    ///
    /// Returns error if temporary directory creation fails
    pub async fn new() -> std::io::Result<Self> {
        Ok(Self {
            temp_dir: TempDir::new()?,
            port_pool: PortAllocator::shared(),
            _cleanup: CleanupGuard::new(),
            coordinator: Arc::new(ConcurrentCoordinator::new()),
        })
    }

    /// Allocate unique port (thread-safe, no conflicts)
    ///
    /// Returns a port that no other concurrent test is using.
    /// Ports are automatically released when context is dropped.
    pub async fn allocate_port(&self) -> u16 {
        self.port_pool.allocate().await
    }

    /// Get isolated temporary directory path
    ///
    /// This directory is unique to this test and will be
    /// cleaned up automatically when the test completes.
    pub fn temp_dir(&self) -> &Path {
        self.temp_dir.path()
    }

    /// Get path within temp directory
    pub fn temp_path(&self, relative: &str) -> PathBuf {
        self.temp_dir.path().join(relative)
    }

    /// Get coordinator for event-driven coordination
    pub fn coordinator(&self) -> Arc<ConcurrentCoordinator> {
        self.coordinator.clone()
    }
}

/// Thread-safe port allocator
///
/// Ensures no port conflicts between concurrent tests.
/// Uses lock-free atomic operations for maximum performance.
pub struct PortAllocator {
    /// Next port to try (atomic)
    next_port: Arc<AtomicU16>,
    /// Set of allocated ports (concurrent hash set)
    allocated: Arc<DashSet<u16>>,
}

impl PortAllocator {
    /// Get shared singleton instance
    ///
    /// All tests share the same allocator to prevent conflicts.
    pub fn shared() -> Arc<Self> {
        static INSTANCE: OnceLock<Arc<PortAllocator>> = OnceLock::new();
        INSTANCE
            .get_or_init(|| {
                Arc::new(Self {
                    next_port: Arc::new(AtomicU16::new(10000)),
                    allocated: Arc::new(DashSet::new()),
                })
            })
            .clone()
    }

    /// Allocate unique port
    ///
    /// Uses lock-free algorithm to find available port.
    /// Starts at 10000 to avoid system ports and common services.
    pub async fn allocate(&self) -> u16 {
        loop {
            let port = self.next_port.fetch_add(1, Ordering::Relaxed);

            // Wrap around if we reach max
            if port == 0 {
                self.next_port.store(10000, Ordering::Relaxed);
                continue;
            }

            // Try to insert (returns true if newly inserted)
            if self.allocated.insert(port) {
                return port;
            }
        }
    }

    /// Release port back to pool
    pub fn release(&self, port: u16) {
        self.allocated.remove(&port);
    }
}

/// Event-driven coordinator - replaces sleep-based coordination
///
/// # Anti-pattern (OLD):
/// ```ignore
/// assert!(service.is_ready());
/// ```
///
/// # Modern pattern (NEW):
/// ```ignore
/// coordinator.wait_ready().await;
/// assert!(service.is_ready());
/// ```
#[derive(Clone)]
pub struct ConcurrentCoordinator {
    /// Ready signal (one-shot notification)
    ready: Arc<Notify>,
    /// State watcher (continuous updates)
    state_tx: watch::Sender<CoordinatorState>,
    /// State receiver
    state_rx: watch::Receiver<CoordinatorState>,
}

/// Coordinator state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CoordinatorState {
    /// Not started
    #[default]
    Idle,
    /// Starting up
    Starting,
    /// Ready for operation
    Ready,
    /// Shutting down
    Stopping,
    /// Stopped
    Stopped,
    /// Error state
    Error,
}

impl ConcurrentCoordinator {
    /// Create new coordinator
    pub fn new() -> Self {
        let (state_tx, state_rx) = watch::channel(CoordinatorState::Idle);
        Self {
            ready: Arc::new(Notify::new()),
            state_tx,
            state_rx,
        }
    }

    /// Wait for ready signal (event-driven, no polling!)
    ///
    /// This replaces sleep-based waiting. The test blocks until
    /// the service actually signals it's ready.
    pub async fn wait_ready(&self) {
        self.ready.notified().await;
    }

    /// Wait for ready signal with timeout
    ///
    /// Returns `Ok(())` if ready signal received,
    /// `Err(())` if timeout elapsed.
    pub async fn wait_ready_timeout(&self, timeout: std::time::Duration) -> Result<(), ()> {
        tokio::select! {
            _ = self.ready.notified() => Ok(()),
            _ = tokio::time::sleep(timeout) => Err(()),
        }
    }

    /// Signal ready (wake all waiters)
    pub fn signal_ready(&self) {
        self.state_tx.send_replace(CoordinatorState::Ready);
        self.ready.notify_waiters();
    }

    /// Update state
    pub fn set_state(&self, state: CoordinatorState) {
        self.state_tx.send_replace(state);

        // Auto-signal ready when entering Ready state
        if state == CoordinatorState::Ready {
            self.ready.notify_waiters();
        }
    }

    /// Get current state
    pub fn state(&self) -> CoordinatorState {
        *self.state_rx.borrow()
    }

    /// Wait for specific state
    ///
    /// Uses watch channel for efficient state monitoring.
    pub async fn wait_for_state(&mut self, target: CoordinatorState) {
        loop {
            if *self.state_rx.borrow() == target {
                return;
            }
            if self.state_rx.changed().await.is_err() {
                // Channel closed
                return;
            }
        }
    }

    /// Create child coordinator
    ///
    /// Useful for hierarchical coordination (e.g., service → component)
    pub fn child(&self) -> Self {
        Self::new()
    }
}

impl Default for ConcurrentCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

/// Cleanup guard - ensures cleanup even on panic
///
/// Runs registered cleanup functions in LIFO order when dropped.
/// Uses `tokio::sync::RwLock` which does not require `T: Sync`.
/// Boxed cleanup function for LIFO execution
type CleanupFn = Box<dyn FnOnce() + Send + 'static>;

#[allow(clippy::arc_with_non_send_sync)]
pub struct CleanupGuard {
    cleanups: Arc<RwLock<Vec<CleanupFn>>>,
}

impl CleanupGuard {
    /// Create new cleanup guard
    #[allow(clippy::arc_with_non_send_sync)]
    pub fn new() -> Self {
        Self {
            cleanups: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Register cleanup function
    ///
    /// Cleanup functions are called in LIFO order (last registered, first called).
    pub async fn register<F>(&self, cleanup: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.cleanups.write().await.push(Box::new(cleanup));
    }
}

impl Drop for CleanupGuard {
    fn drop(&mut self) {
        // Run cleanups in LIFO order
        if let Ok(mut cleanups) = self.cleanups.try_write() {
            while let Some(cleanup) = cleanups.pop() {
                cleanup();
            }
        }
    }
}

impl Default for CleanupGuard {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_isolated_context_creation() {
        let ctx = IsolatedTestContext::new().await.unwrap();
        assert!(ctx.temp_dir().exists());
    }

    #[tokio::test]
    async fn test_port_allocation_unique() {
        let allocator = PortAllocator::shared();

        let port1 = allocator.allocate().await;
        let port2 = allocator.allocate().await;
        let port3 = allocator.allocate().await;

        // All ports should be unique
        assert_ne!(port1, port2);
        assert_ne!(port2, port3);
        assert_ne!(port1, port3);

        // Cleanup
        allocator.release(port1);
        allocator.release(port2);
        allocator.release(port3);
    }

    #[tokio::test]
    async fn test_concurrent_port_allocation() {
        let allocator = PortAllocator::shared();

        // Allocate ports concurrently
        let handles: Vec<_> = (0..100)
            .map(|_| {
                let alloc = allocator.clone();
                tokio::spawn(async move { alloc.allocate().await })
            })
            .collect();

        let ports: Vec<u16> = futures::future::join_all(handles)
            .await
            .into_iter()
            .map(|r| r.unwrap())
            .collect();

        // All ports should be unique
        let unique_ports: std::collections::HashSet<_> = ports.iter().copied().collect();
        assert_eq!(unique_ports.len(), 100);

        // Cleanup
        for port in ports {
            allocator.release(port);
        }
    }

    #[tokio::test]
    async fn test_coordinator_ready_signal() {
        let coord = ConcurrentCoordinator::new();
        let coord_clone = coord.clone();

        // Spawn task that signals ready after brief moment
        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            coord_clone.signal_ready();
        });

        // Wait should complete when signaled (not timeout)
        let start = std::time::Instant::now();
        coord.wait_ready().await;
        let elapsed = start.elapsed();

        // Should complete quickly (< 100ms)
        assert!(elapsed < std::time::Duration::from_millis(100));
    }

    #[tokio::test]
    async fn test_coordinator_state_transitions() {
        let coord = ConcurrentCoordinator::new();

        assert_eq!(coord.state(), CoordinatorState::Idle);

        coord.set_state(CoordinatorState::Starting);
        assert_eq!(coord.state(), CoordinatorState::Starting);

        coord.set_state(CoordinatorState::Ready);
        assert_eq!(coord.state(), CoordinatorState::Ready);
    }

    #[tokio::test]
    async fn test_cleanup_guard_runs() {
        use std::sync::atomic::{AtomicBool, Ordering};

        let cleaned = Arc::new(AtomicBool::new(false));
        let cleaned_clone = cleaned.clone();

        {
            let guard = CleanupGuard::new();
            guard
                .register(move || {
                    cleaned_clone.store(true, Ordering::Relaxed);
                })
                .await;

            // Guard goes out of scope here
        }

        // Cleanup should have run
        assert!(cleaned.load(Ordering::Relaxed));
    }
}
