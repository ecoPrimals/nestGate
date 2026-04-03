// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Isolated Test Runner
//!
//! Provides dedicated Tokio runtimes for test isolation at the runtime level.
//! Prevents runtime contention and ensures tests run in complete isolation.
//!
//! ## Features
//! - Dedicated runtime per test
//! - Configurable runtime parameters (threads, stack size)
//! - Integration with TestResourceManager
//! - Integration with IsolatedEnvironment
//! - Support for both sync and async tests
//! - Automatic cleanup (RAII pattern)
//!
//! ## Usage
//! ```rust,ignore
//! let runner = IsolatedTestRunner::new("my_test");
//!
//! runner.run_async(async {
//!     // Test code here - runs in dedicated runtime
//!     Ok(())
//! }).unwrap();
//! ```

use std::future::Future;
use std::panic::{AssertUnwindSafe, catch_unwind};
use tokio::runtime::{Builder, Runtime};

use super::env_isolation::IsolatedEnvironment;
use super::test_resource_manager::TestResourceManager;

/// Runtime configuration for isolated test execution
#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    /// Number of worker threads (None = number of CPUs)
    pub worker_threads: Option<usize>,

    /// Thread stack size in bytes (None = default)
    pub thread_stack_size: Option<usize>,

    /// Thread name prefix
    pub thread_name: String,

    /// Enable I/O driver
    pub enable_io: bool,

    /// Enable time driver
    pub enable_time: bool,

    /// Maximum blocking threads
    pub max_blocking_threads: usize,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            worker_threads: Some(2), // 2 threads for most tests
            thread_stack_size: None, // Use default
            thread_name: "test-worker".to_string(),
            enable_io: true,
            enable_time: true,
            max_blocking_threads: 512,
        }
    }
}

impl RuntimeConfig {
    /// Create a minimal runtime configuration for lightweight tests
    pub fn minimal() -> Self {
        Self {
            worker_threads: Some(1),
            thread_stack_size: Some(2 * 1024 * 1024), // 2MB stack
            thread_name: "minimal-test".to_string(),
            enable_io: false,
            enable_time: false,
            max_blocking_threads: 16,
        }
    }

    /// Create a high-performance runtime configuration for heavy tests
    pub fn high_performance() -> Self {
        Self {
            worker_threads: None,                     // Use all CPUs
            thread_stack_size: Some(8 * 1024 * 1024), // 8MB stack
            thread_name: "perf-test".to_string(),
            enable_io: true,
            enable_time: true,
            max_blocking_threads: 1024,
        }
    }

    /// Set number of worker threads
    pub fn with_worker_threads(mut self, threads: usize) -> Self {
        self.worker_threads = Some(threads);
        self
    }

    /// Set thread stack size
    pub fn with_stack_size(mut self, size: usize) -> Self {
        self.thread_stack_size = Some(size);
        self
    }

    /// Set thread name prefix
    pub fn with_thread_name(mut self, name: &str) -> Self {
        self.thread_name = name.to_string();
        self
    }

    /// Build a Tokio runtime from this configuration
    fn build_runtime(&self) -> Result<Runtime, std::io::Error> {
        let mut builder = Builder::new_multi_thread();

        if let Some(threads) = self.worker_threads {
            builder.worker_threads(threads);
        }

        if let Some(stack_size) = self.thread_stack_size {
            builder.thread_stack_size(stack_size);
        }

        builder
            .thread_name(&self.thread_name)
            .max_blocking_threads(self.max_blocking_threads);

        if self.enable_io {
            builder.enable_io();
        }

        if self.enable_time {
            builder.enable_time();
        }

        builder.build()
    }
}

/// Isolated Test Runner
///
/// Provides a dedicated Tokio runtime for test execution, ensuring complete
/// isolation from other tests. Automatically cleans up resources on drop.
///
/// Can optionally integrate with TestResourceManager and IsolatedEnvironment
/// for comprehensive test isolation.
pub struct IsolatedTestRunner {
    /// Test name for identification
    test_name: String,

    /// Dedicated Tokio runtime
    runtime: Runtime,

    /// Optional resource manager
    resource_manager: Option<TestResourceManager>,

    /// Optional environment isolation
    env_isolation: Option<IsolatedEnvironment>,
}

impl IsolatedTestRunner {
    /// Create a new isolated test runner with default configuration
    ///
    /// # Arguments
    /// * `test_name` - Unique name for the test
    ///
    /// # Example
    /// ```rust
    /// use crate::common::isolated_test_runner::IsolatedTestRunner;
    ///
    /// let runner = IsolatedTestRunner::new("my_test");
    /// ```
    pub fn new(test_name: &str) -> Self {
        let config = RuntimeConfig::default();
        Self::with_config(test_name, config)
    }

    /// Create a new isolated test runner with custom configuration
    ///
    /// # Arguments
    /// * `test_name` - Unique name for the test
    /// * `config` - Runtime configuration
    pub fn with_config(test_name: &str, config: RuntimeConfig) -> Self {
        let runtime = config
            .build_runtime()
            .expect("Failed to create Tokio runtime");

        Self {
            test_name: test_name.to_string(),
            runtime,
            resource_manager: None,
            env_isolation: None,
        }
    }

    /// Create a minimal isolated test runner for lightweight tests
    pub fn minimal(test_name: &str) -> Self {
        Self::with_config(test_name, RuntimeConfig::minimal())
    }

    /// Create a high-performance isolated test runner for heavy tests
    pub fn high_performance(test_name: &str) -> Self {
        Self::with_config(test_name, RuntimeConfig::high_performance())
    }

    /// Enable resource management for this test
    pub fn with_resource_management(mut self, manager: TestResourceManager) -> Self {
        self.resource_manager = Some(manager);
        self
    }

    /// Enable environment isolation for this test
    pub fn with_env_isolation(mut self, isolation: IsolatedEnvironment) -> Self {
        self.env_isolation = Some(isolation);
        self
    }

    /// Run an async test in the isolated runtime
    ///
    /// # Arguments
    /// * `test_fn` - Async function to execute
    ///
    /// # Returns
    /// Result of the test execution
    ///
    /// # Example
    /// ```rust
    /// use crate::common::isolated_test_runner::IsolatedTestRunner;
    ///
    /// let runner = IsolatedTestRunner::new("async_test");
    /// runner.run_async(async {
    ///     // Test code here
    ///     Ok::<(), Box<dyn std::error::Error>>(())
    /// }).unwrap();
    /// ```
    pub fn run_async<F, T, E>(
        &self,
        test_fn: F,
    ) -> Result<T, Box<dyn std::error::Error + Send + Sync>>
    where
        F: Future<Output = Result<T, E>> + Send + 'static,
        T: Send + 'static,
        E: Into<Box<dyn std::error::Error + Send + Sync>> + Send + 'static,
    {
        // Execute in the isolated runtime
        let result = self
            .runtime
            .block_on(async { test_fn.await.map_err(|e| e.into()) });

        result
    }

    /// Run a synchronous test in the isolated runtime's context
    ///
    /// # Arguments
    /// * `test_fn` - Synchronous function to execute
    ///
    /// # Returns
    /// Result of the test execution
    ///
    /// # Example
    /// ```rust
    /// use crate::common::isolated_test_runner::IsolatedTestRunner;
    ///
    /// let runner = IsolatedTestRunner::new("sync_test");
    /// runner.run_sync(|| {
    ///     // Test code here
    ///     Ok::<(), Box<dyn std::error::Error>>(())
    /// }).unwrap();
    /// ```
    pub fn run_sync<F, T, E>(
        &self,
        test_fn: F,
    ) -> Result<T, Box<dyn std::error::Error + Send + Sync>>
    where
        F: FnOnce() -> Result<T, E> + Send + 'static,
        T: Send + 'static,
        E: Into<Box<dyn std::error::Error + Send + Sync>> + Send + 'static,
    {
        // Spawn blocking task in the isolated runtime
        let result = self.runtime.block_on(async {
            tokio::task::spawn_blocking(test_fn)
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?
                .map_err(|e| e.into())
        });

        result
    }

    /// Run a test with panic catching
    ///
    /// # Arguments
    /// * `test_fn` - Async function to execute
    ///
    /// # Returns
    /// Result with panic information if test panicked
    pub fn run_with_panic_catch<F, T, E>(
        &self,
        test_fn: F,
    ) -> Result<T, Box<dyn std::error::Error + Send + Sync>>
    where
        F: Future<Output = Result<T, E>> + Send + 'static,
        T: Send + 'static,
        E: Into<Box<dyn std::error::Error + Send + Sync>> + Send + 'static,
    {
        let result = catch_unwind(AssertUnwindSafe(|| self.run_async(test_fn)));

        match result {
            Ok(inner_result) => inner_result,
            Err(panic_info) => {
                let panic_msg = if let Some(s) = panic_info.downcast_ref::<String>() {
                    s.clone()
                } else if let Some(s) = panic_info.downcast_ref::<&str>() {
                    s.to_string()
                } else {
                    "Unknown panic".to_string()
                };

                Err(format!("Test '{}' panicked: {}", self.test_name, panic_msg).into())
            }
        }
    }

    /// Get the test name
    pub fn test_name(&self) -> &str {
        &self.test_name
    }

    /// Get a reference to the resource manager if enabled
    pub fn resource_manager(&self) -> Option<&TestResourceManager> {
        self.resource_manager.as_ref()
    }

    /// Get a mutable reference to the resource manager if enabled
    pub fn resource_manager_mut(&mut self) -> Option<&mut TestResourceManager> {
        self.resource_manager.as_mut()
    }

    /// Get a reference to the environment isolation if enabled
    pub fn env_isolation(&self) -> Option<&IsolatedEnvironment> {
        self.env_isolation.as_ref()
    }

    /// Get a mutable reference to the environment isolation if enabled
    pub fn env_isolation_mut(&mut self) -> Option<&mut IsolatedEnvironment> {
        self.env_isolation.as_mut()
    }
}

impl Drop for IsolatedTestRunner {
    fn drop(&mut self) {
        // Runtime will be shut down automatically
        // Resource manager and environment isolation will clean up via their Drop impls
    }
}

/// Helper macro for creating isolated test runners
///
/// # Example
/// ```rust
/// isolated_test!("my_test", async {
///     // Test code here
///     Ok::<(), Box<dyn std::error::Error>>(())
/// });
/// ```
#[macro_export]
macro_rules! isolated_test {
    ($name:expr, $test_fn:expr) => {{
        let runner = $crate::common::isolated_test_runner::IsolatedTestRunner::new($name);
        runner.run_async($test_fn)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Duration;

    #[test]
    fn test_runner_creation() {
        let runner = IsolatedTestRunner::new("test_creation");
        assert_eq!(runner.test_name(), "test_creation");
    }

    #[test]
    fn test_minimal_runner() {
        let runner = IsolatedTestRunner::minimal("test_minimal");
        assert_eq!(runner.test_name(), "test_minimal");
    }

    #[test]
    fn test_high_performance_runner() {
        let runner = IsolatedTestRunner::high_performance("test_high_perf");
        assert_eq!(runner.test_name(), "test_high_perf");
    }

    #[test]
    fn test_run_async_success() {
        let runner = IsolatedTestRunner::new("test_async_success");

        let result =
            runner.run_async(async { Ok::<i32, Box<dyn std::error::Error + Send + Sync>>(42) });

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_run_async_error() {
        let runner = IsolatedTestRunner::new("test_async_error");

        let result = runner.run_async(async {
            Err::<i32, Box<dyn std::error::Error + Send + Sync>>("Test error".into())
        });

        assert!(result.is_err());
    }

    #[test]
    fn test_run_sync_success() {
        let runner = IsolatedTestRunner::new("test_sync_success");

        let result = runner.run_sync(|| Ok::<i32, Box<dyn std::error::Error + Send + Sync>>(42));

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_run_sync_error() {
        let runner = IsolatedTestRunner::new("test_sync_error");

        let result = runner
            .run_sync(|| Err::<i32, Box<dyn std::error::Error + Send + Sync>>("Test error".into()));

        assert!(result.is_err());
    }

    #[test]
    fn test_runtime_isolation() {
        // Test that each runner has its own isolated runtime
        let counter = Arc::new(AtomicUsize::new(0));

        let runner1 = IsolatedTestRunner::new("test_isolation_1");
        let runner2 = IsolatedTestRunner::new("test_isolation_2");

        let counter1 = Arc::clone(&counter);
        let result1 = runner1.run_async(async move {
            counter1.fetch_add(1, Ordering::SeqCst);
            tokio::time::sleep(Duration::from_millis(10)).await;
            Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
        });

        let counter2 = Arc::clone(&counter);
        let result2 = runner2.run_async(async move {
            counter2.fetch_add(1, Ordering::SeqCst);
            tokio::time::sleep(Duration::from_millis(10)).await;
            Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
        });

        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert_eq!(counter.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn test_with_resource_management() {
        let resource_manager = TestResourceManager::new("test_resources");
        let mut runner = IsolatedTestRunner::new("test_with_resources")
            .with_resource_management(resource_manager);

        assert!(runner.resource_manager().is_some());

        if let Some(manager) = runner.resource_manager_mut() {
            manager.allocate_memory(10);
            assert_eq!(manager.usage().memory_mb, 10);
        }
    }

    #[test]
    fn test_with_env_isolation() {
        let env_isolation = IsolatedEnvironment::new("test_env");
        let mut runner = IsolatedTestRunner::new("test_with_env").with_env_isolation(env_isolation);

        assert!(runner.env_isolation().is_some());

        if let Some(env) = runner.env_isolation_mut() {
            env.set("TEST_VAR", "test_value");
        }
    }

    #[test]
    fn test_runtime_config() {
        let config = RuntimeConfig::default()
            .with_worker_threads(4)
            .with_stack_size(4 * 1024 * 1024)
            .with_thread_name("custom-test");

        assert_eq!(config.worker_threads, Some(4));
        assert_eq!(config.thread_stack_size, Some(4 * 1024 * 1024));
        assert_eq!(config.thread_name, "custom-test");
    }
}
