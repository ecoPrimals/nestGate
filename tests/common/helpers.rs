/// Clean test helpers that actually work
/// Replaces the broken test_framework references
use super::{CleanTestConfig, NestGateError, Result, TestResult};
use std::time::{Duration, Instant};
use tokio::time::timeout;

/// Test execution helpers
pub struct TestHelpers;

impl TestHelpers {
    /// Run a test function with timeout and error handling
    pub async fn run_test_with_timeout<F, Fut>(
        test_name: &str,
        test_fn: F,
        timeout_duration: Duration,
    ) -> TestResult
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<()>>,
    {
        let start = Instant::now();

        match timeout(timeout_duration, test_fn()).await {
            Ok(Ok(())) => TestResult::success(test_name.to_string(), start.elapsed()),
            Ok(Err(e)) => TestResult::failure(
                test_name.to_string(),
                start.elapsed(),
                format!("Test failed: {}", e),
            ),
            Err(_) => TestResult::failure(
                test_name.to_string(),
                start.elapsed(),
                format!("Test timed out after {:?}", timeout_duration),
            ),
        }
    }

    /// Run multiple tests concurrently
    pub async fn run_concurrent_tests<F, Fut>(
        test_name_prefix: &str,
        test_fn: F,
        concurrent_count: usize,
        individual_timeout: Duration,
    ) -> Vec<TestResult>
    where
        F: Fn(usize) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<()>> + Send,
    {
        let test_fn = std::sync::Arc::new(test_fn);
        let mut handles = Vec::new();

        for i in 0..concurrent_count {
            let test_name = format!("{}_{}", test_name_prefix, i);
            let test_fn_clone = test_fn.clone();

            let handle = tokio::spawn(async move {
                Self::run_test_with_timeout(&test_name, || test_fn_clone(i), individual_timeout)
                    .await
            });

            handles.push(handle);
        }

        let mut results = Vec::new();
        for handle in handles {
            match handle.await {
                Ok(result) => results.push(result),
                Err(e) => results.push(TestResult::failure(
                    "concurrent_test".to_string(),
                    Duration::from_secs(0),
                    format!("Task join error: {}", e),
                )),
            }
        }

        results
    }

    /// Simple assertion helper
    pub fn assert_test(condition: bool, message: &str) -> Result<()> {
        if condition {
            Ok(())
        } else {
            Err(NestGateError::internal_error(
                format!("Assertion failed: {}", message),
                "test_helpers".to_string(),
            ))
        }
    }

    /// Assert two values are equal
    pub fn assert_eq<T: std::fmt::Debug + PartialEq>(
        left: T,
        right: T,
        message: &str,
    ) -> Result<()> {
        if left == right {
            Ok(())
        } else {
            Err(NestGateError::internal_error(
                format!(
                    "Assertion failed: {} - Expected {:?}, got {:?}",
                    message, right, left
                ),
                "test_helpers".to_string(),
            ))
        }
    }

    /// Create test data for performance testing
    pub fn create_test_data(size: usize) -> Vec<String> {
        (0..size).map(|i| format!("test_data_item_{}", i)).collect()
    }

    /// Simulate work with configurable duration
    pub async fn simulate_work(duration: Duration) -> Result<String> {
        tokio::time::sleep(duration).await;
        Ok("work_completed".to_string())
    }

    /// Simulate work that might fail (deterministic for testing)
    pub async fn simulate_fallible_work(duration: Duration, should_fail: bool) -> Result<String> {
        tokio::time::sleep(duration).await;

        if should_fail {
            Err(NestGateError::internal_error(
                "Simulated failure".to_string(),
                "test_simulation".to_string(),
            ))
        } else {
            Ok("work_completed".to_string())
        }
    }

    /// Clean up test resources
    pub async fn cleanup_test_resources(resource_names: Vec<String>) -> Result<()> {
        // Simulate cleanup work
        for name in resource_names {
            tokio::time::sleep(Duration::from_millis(1)).await;
            tracing::debug!("Cleaned up test resource: {}", name);
        }
        Ok(())
    }
}

/// Test setup and teardown helpers
pub struct TestSetup;

impl TestSetup {
    /// Initialize test environment
    pub async fn initialize(config: &CleanTestConfig) -> Result<TestEnvironment> {
        tracing::info!("🏗️ Initializing test environment: {}", config.name);

        // Simulate setup work
        tokio::time::sleep(Duration::from_millis(10)).await;

        Ok(TestEnvironment {
            name: config.name.clone(),
            initialized_at: Instant::now(),
            resources: Vec::new(),
        })
    }

    /// Clean up test environment
    pub async fn cleanup(env: TestEnvironment) -> Result<()> {
        tracing::info!("🧹 Cleaning up test environment: {}", env.name);

        // Clean up resources
        TestHelpers::cleanup_test_resources(env.resources).await?;

        Ok(())
    }
}

/// Test environment tracking
pub struct TestEnvironment {
    pub name: String,
    pub initialized_at: Instant,
    pub resources: Vec<String>,
}

impl TestEnvironment {
    pub fn add_resource(&mut self, resource_name: String) {
        self.resources.push(resource_name);
    }

    pub fn uptime(&self) -> Duration {
        self.initialized_at.elapsed()
    }
}
