//! Test Resource Manager
//!
//! Dynamic resource management for tests to prevent resource exhaustion.
//! Tracks memory, CPU, and I/O usage per test and enforces quotas.
//!
//! ## Features
//! - Memory tracking and limits
//! - CPU time tracking
//! - I/O operation counting
//! - Automatic resource cleanup
//! - Resource quota enforcement
//!
//! ## Usage
//! ```rust,ignore
//! let mut manager = TestResourceManager::new("my_test");
//! manager.set_quota(ResourceQuota::default().with_max_memory_mb(100));
//!
//! // Test code here - manager tracks resource usage
//! // Automatic cleanup on drop
//! ```

use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};
use std::time::{Duration, Instant};

/// Global resource manager lock to serialize resource tracking
static RESOURCE_MANAGER_LOCK: Mutex<()> = Mutex::new(());

/// Global resource tracking state
static GLOBAL_RESOURCES: Mutex<Option<GlobalResourceState>> = Mutex::new(None);

/// Global resource tracking state
#[derive(Debug, Clone)]
struct GlobalResourceState {
    active_tests: HashMap<String, ResourceUsage>,
    total_allocated_memory_mb: u64,
    peak_concurrent_tests: usize,
}

impl GlobalResourceState {
    fn new() -> Self {
        Self {
            active_tests: HashMap::new(),
            total_allocated_memory_mb: 0,
            peak_concurrent_tests: 0,
        }
    }
}

/// Resource quota configuration
#[derive(Debug, Clone, Copy)]
pub struct ResourceQuota {
    /// Maximum memory usage in MB (0 = unlimited)
    pub max_memory_mb: u64,

    /// Maximum CPU time in seconds (0 = unlimited)
    pub max_cpu_seconds: u64,

    /// Maximum I/O operations (0 = unlimited)
    pub max_io_ops: u64,

    /// Maximum test duration in seconds (0 = unlimited)
    pub max_duration_seconds: u64,
}

impl Default for ResourceQuota {
    fn default() -> Self {
        Self {
            max_memory_mb: 500,        // 500 MB per test
            max_cpu_seconds: 60,       // 1 minute CPU time
            max_io_ops: 10000,         // 10K I/O operations
            max_duration_seconds: 120, // 2 minutes wall time
        }
    }
}

impl ResourceQuota {
    /// Create an unlimited quota (for performance tests)
    pub fn unlimited() -> Self {
        Self {
            max_memory_mb: 0,
            max_cpu_seconds: 0,
            max_io_ops: 0,
            max_duration_seconds: 0,
        }
    }

    /// Set maximum memory in MB
    pub fn with_max_memory_mb(mut self, mb: u64) -> Self {
        self.max_memory_mb = mb;
        self
    }

    /// Set maximum CPU time in seconds
    pub fn with_max_cpu_seconds(mut self, seconds: u64) -> Self {
        self.max_cpu_seconds = seconds;
        self
    }

    /// Set maximum I/O operations
    pub fn with_max_io_ops(mut self, ops: u64) -> Self {
        self.max_io_ops = ops;
        self
    }

    /// Set maximum duration in seconds
    pub fn with_max_duration_seconds(mut self, seconds: u64) -> Self {
        self.max_duration_seconds = seconds;
        self
    }
}

/// Current resource usage tracking
#[derive(Debug, Clone)]
pub struct ResourceUsage {
    /// Estimated memory usage in MB
    pub memory_mb: u64,

    /// CPU time used in milliseconds
    pub cpu_ms: u64,

    /// Number of I/O operations performed
    pub io_ops: u64,

    /// Test start time
    pub start_time: Instant,

    /// Peak memory usage
    pub peak_memory_mb: u64,
}

impl Default for ResourceUsage {
    fn default() -> Self {
        Self {
            memory_mb: 0,
            cpu_ms: 0,
            io_ops: 0,
            start_time: Instant::now(),
            peak_memory_mb: 0,
        }
    }
}

impl ResourceUsage {
    /// Get elapsed time since test start
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Check if usage exceeds quota
    pub fn exceeds_quota(&self, quota: &ResourceQuota) -> Option<String> {
        // Check memory
        if quota.max_memory_mb > 0 && self.memory_mb > quota.max_memory_mb {
            return Some(format!(
                "Memory limit exceeded: {} MB > {} MB",
                self.memory_mb, quota.max_memory_mb
            ));
        }

        // Check CPU time
        if quota.max_cpu_seconds > 0 && self.cpu_ms > quota.max_cpu_seconds * 1000 {
            return Some(format!(
                "CPU time limit exceeded: {} ms > {} ms",
                self.cpu_ms,
                quota.max_cpu_seconds * 1000
            ));
        }

        // Check I/O operations
        if quota.max_io_ops > 0 && self.io_ops > quota.max_io_ops {
            return Some(format!(
                "I/O operations limit exceeded: {} > {}",
                self.io_ops, quota.max_io_ops
            ));
        }

        // Check duration
        let elapsed_secs = self.elapsed().as_secs();
        if quota.max_duration_seconds > 0 && elapsed_secs > quota.max_duration_seconds {
            return Some(format!(
                "Duration limit exceeded: {} s > {} s",
                elapsed_secs, quota.max_duration_seconds
            ));
        }

        None
    }
}

/// Test Resource Manager
///
/// Tracks and enforces resource quotas for individual tests.
/// Automatically cleans up resources on drop (RAII pattern).
pub struct TestResourceManager {
    /// Test name for identification
    test_name: String,

    /// Resource quota for this test
    quota: ResourceQuota,

    /// Current resource usage
    usage: ResourceUsage,

    /// Whether to panic on quota violation
    enforce_quota: bool,

    /// Lock held for the duration of the test
    _lock: Option<MutexGuard<'static, ()>>,
}

impl TestResourceManager {
    /// Create a new resource manager for a test
    ///
    /// # Arguments
    /// * `test_name` - Unique name for the test
    ///
    /// # Example
    /// ```rust
    /// use crate::common::test_resource_manager::TestResourceManager;
    ///
    /// let manager = TestResourceManager::new("my_test");
    /// ```
    pub fn new(test_name: &str) -> Self {
        // Initialize global state if needed
        let mut global = GLOBAL_RESOURCES
            .lock()
            .expect("Failed to lock global resources");
        if global.is_none() {
            *global = Some(GlobalResourceState::new());
        }

        // Register this test
        if let Some(ref mut state) = *global {
            state
                .active_tests
                .insert(test_name.to_string(), ResourceUsage::default());
            state.peak_concurrent_tests = state.peak_concurrent_tests.max(state.active_tests.len());
        }
        drop(global);

        Self {
            test_name: test_name.to_string(),
            quota: ResourceQuota::default(),
            usage: ResourceUsage::default(),
            enforce_quota: false, // Default to non-enforcing (warnings only)
            _lock: None,
        }
    }

    /// Create a new resource manager with serialization
    ///
    /// This will acquire a global lock, ensuring only one test runs at a time.
    /// Use this for resource-intensive tests that need exclusive access.
    pub fn new_serialized(test_name: &str) -> Self {
        let lock = RESOURCE_MANAGER_LOCK
            .lock()
            .expect("Failed to acquire resource lock");

        let mut manager = Self::new(test_name);
        manager._lock = Some(lock);
        manager
    }

    /// Set the resource quota for this test
    pub fn set_quota(&mut self, quota: ResourceQuota) -> &mut Self {
        self.quota = quota;
        self
    }

    /// Enable strict quota enforcement (panic on violation)
    pub fn enforce_quota(&mut self, enforce: bool) -> &mut Self {
        self.enforce_quota = enforce;
        self
    }

    /// Record memory allocation
    pub fn allocate_memory(&mut self, size_mb: u64) {
        self.usage.memory_mb += size_mb;
        self.usage.peak_memory_mb = self.usage.peak_memory_mb.max(self.usage.memory_mb);

        // Update global tracking
        if let Ok(mut global) = GLOBAL_RESOURCES.lock() {
            if let Some(ref mut state) = *global {
                state.total_allocated_memory_mb += size_mb;
            }
        }

        self.check_quota();
    }

    /// Record memory deallocation
    pub fn deallocate_memory(&mut self, size_mb: u64) {
        self.usage.memory_mb = self.usage.memory_mb.saturating_sub(size_mb);

        // Update global tracking
        if let Ok(mut global) = GLOBAL_RESOURCES.lock() {
            if let Some(ref mut state) = *global {
                state.total_allocated_memory_mb =
                    state.total_allocated_memory_mb.saturating_sub(size_mb);
            }
        }
    }

    /// Record CPU time used (in milliseconds)
    pub fn record_cpu_time(&mut self, ms: u64) {
        self.usage.cpu_ms += ms;
        self.check_quota();
    }

    /// Record an I/O operation
    pub fn record_io_op(&mut self) {
        self.usage.io_ops += 1;
        self.check_quota();
    }

    /// Record multiple I/O operations
    pub fn record_io_ops(&mut self, count: u64) {
        self.usage.io_ops += count;
        self.check_quota();
    }

    /// Get current resource usage
    pub fn usage(&self) -> &ResourceUsage {
        &self.usage
    }

    /// Get resource quota
    pub fn quota(&self) -> &ResourceQuota {
        &self.quota
    }

    /// Check if quota is exceeded and handle accordingly
    fn check_quota(&self) {
        if let Some(violation) = self.usage.exceeds_quota(&self.quota) {
            if self.enforce_quota {
                panic!(
                    "Test '{}': Resource quota exceeded: {}",
                    self.test_name, violation
                );
            } else {
                eprintln!("⚠️  Test '{}': Warning: {}", self.test_name, violation);
            }
        }
    }

    /// Get resource usage summary
    pub fn summary(&self) -> String {
        format!(
            "Test '{}': Memory: {} MB (peak: {} MB), CPU: {} ms, I/O: {} ops, Duration: {:?}",
            self.test_name,
            self.usage.memory_mb,
            self.usage.peak_memory_mb,
            self.usage.cpu_ms,
            self.usage.io_ops,
            self.usage.elapsed()
        )
    }
}

impl Drop for TestResourceManager {
    fn drop(&mut self) {
        // Print summary if test used significant resources
        if self.usage.memory_mb > 100 || self.usage.cpu_ms > 1000 || self.usage.io_ops > 1000 {
            println!("📊 {}", self.summary());
        }

        // Unregister from global state
        if let Ok(mut global) = GLOBAL_RESOURCES.lock() {
            if let Some(ref mut state) = *global {
                state.active_tests.remove(&self.test_name);
            }
        }
    }
}

/// Get global resource statistics
pub fn global_resource_stats() -> String {
    if let Ok(global) = GLOBAL_RESOURCES.lock() {
        if let Some(ref state) = *global {
            format!(
                "Global Resources: Active tests: {}, Total memory: {} MB, Peak concurrent: {}",
                state.active_tests.len(),
                state.total_allocated_memory_mb,
                state.peak_concurrent_tests
            )
        } else {
            "Global Resources: Not initialized".to_string()
        }
    } else {
        "Global Resources: Lock failed".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_manager_creation() {
        let manager = TestResourceManager::new("test_creation");
        assert_eq!(manager.test_name, "test_creation");
        assert_eq!(manager.usage.memory_mb, 0);
        assert_eq!(manager.usage.cpu_ms, 0);
        assert_eq!(manager.usage.io_ops, 0);
    }

    #[test]
    fn test_memory_tracking() {
        let mut manager = TestResourceManager::new("test_memory");

        manager.allocate_memory(50);
        assert_eq!(manager.usage.memory_mb, 50);
        assert_eq!(manager.usage.peak_memory_mb, 50);

        manager.allocate_memory(30);
        assert_eq!(manager.usage.memory_mb, 80);
        assert_eq!(manager.usage.peak_memory_mb, 80);

        manager.deallocate_memory(40);
        assert_eq!(manager.usage.memory_mb, 40);
        assert_eq!(manager.usage.peak_memory_mb, 80); // Peak remains
    }

    #[test]
    fn test_cpu_time_tracking() {
        let mut manager = TestResourceManager::new("test_cpu");

        manager.record_cpu_time(100);
        assert_eq!(manager.usage.cpu_ms, 100);

        manager.record_cpu_time(50);
        assert_eq!(manager.usage.cpu_ms, 150);
    }

    #[test]
    fn test_io_tracking() {
        let mut manager = TestResourceManager::new("test_io");

        manager.record_io_op();
        assert_eq!(manager.usage.io_ops, 1);

        manager.record_io_ops(10);
        assert_eq!(manager.usage.io_ops, 11);
    }

    #[test]
    fn test_quota_violation_detection() {
        let mut manager = TestResourceManager::new("test_quota_violation");
        manager.set_quota(ResourceQuota::default().with_max_memory_mb(100));

        manager.allocate_memory(50);
        assert!(manager.usage.exceeds_quota(&manager.quota).is_none());

        manager.allocate_memory(60);
        assert!(manager.usage.exceeds_quota(&manager.quota).is_some());
    }

    #[test]
    fn test_quota_unlimited() {
        let mut manager = TestResourceManager::new("test_unlimited");
        manager.set_quota(ResourceQuota::unlimited());

        // Should not violate unlimited quota
        manager.allocate_memory(10000);
        manager.record_cpu_time(100000);
        manager.record_io_ops(1000000);

        assert!(manager.usage.exceeds_quota(&manager.quota).is_none());
    }

    #[test]
    fn test_resource_summary() {
        let mut manager = TestResourceManager::new("test_summary");
        manager.allocate_memory(100);
        manager.record_cpu_time(500);
        manager.record_io_ops(50);

        let summary = manager.summary();
        assert!(summary.contains("test_summary"));
        assert!(summary.contains("100 MB"));
        assert!(summary.contains("500 ms"));
        assert!(summary.contains("50 ops"));
    }

    #[test]
    fn test_serialized_manager() {
        // This test should run exclusively
        let mut manager = TestResourceManager::new_serialized("test_serialized");
        manager.allocate_memory(10);

        assert_eq!(manager.usage.memory_mb, 10);
        // Lock is held until manager is dropped
    }

    #[test]
    fn test_global_stats() {
        let _manager1 = TestResourceManager::new("test_global_1");
        let _manager2 = TestResourceManager::new("test_global_2");

        let stats = global_resource_stats();
        assert!(stats.contains("Active tests"));
    }
}
