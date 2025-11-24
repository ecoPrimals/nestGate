//! Storage Test Doubles
//!
//! These are PURE TEST MOCKS for testing storage functionality.
//! They simulate various failure conditions and edge cases.
//!
//! ⚠️ IMPORTANT: These are NOT hardware abstractions - they are test infrastructure only.

// use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

use super::TestDoubleConfig;
use nestgate_core::types::StorageTier;

/// Storage test double for unit testing
///
/// This is a pure test mock that simulates storage operations for testing purposes.
/// It can be configured to simulate various failure conditions.
use nestgate_core::canonical_types::StorageTier;
pub struct StorageTestDouble {
    config: TestDoubleConfig,
    operations: Arc<Mutex<Vec<String>>>,
    should_fail_operations: Arc<Mutex<Vec<String>>>,
}

impl StorageTestDouble {
    pub fn new(config: TestDoubleConfig) -> Self {
        Self {
            config,
            operations: Arc::new(Mutex::new(Vec::new())),
            should_fail_operations: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Create a test double that fails specific operations
    pub fn with_failures(operations: Vec<String>) -> Self {
        let mut config = TestDoubleConfig::default();
        config.simulate_failures = true;

        Self {
            config,
            operations: Arc::new(Mutex::new(Vec::new())),
            should_fail_operations: Arc::new(Mutex::new(operations)),
        }
    }

    /// Add an operation to the failure list
    pub fn add_failure(&self, operation: &str) {
        if let Ok(mut failures) = self.should_fail_operations.lock() {
            failures.push(operation.to_string());
        }
    }

    /// Get list of operations that were called
    pub fn get_operations(&self) -> Vec<String> {
        self.operations.lock()?.clone()
    }

    /// Clear operation history
    pub fn clear_operations(&self) {
        if let Ok(mut ops) = self.operations.lock() {
            ops.clear();
        }
    }

    /// Simulate an operation with potential failure
    async fn simulate_operation(&self, operation: &str) -> Result<(), TestStorageError> {
        // Record the operation
        if let Ok(mut ops) = self.operations.lock() {
            ops.push(operation.to_string());
        }

        // Check if this operation should fail
        if let Ok(failures) = self.should_fail_operations.lock() {
            if failures.contains(&operation.to_string()) {
                return Err(TestStorageError::SimulatedFailure(operation.to_string()));
            }
        }

        // Simulate response delay
        if self.config.response_delay_ms > 0 {
            tokio::task::yield_now().await;
        }

        // Log if verbose
        if self.config.verbose_logging {
            println!("TEST DOUBLE: Simulated operation: {}", operation);
        }

        Ok(())
    }
}

/// Mock storage specifically for testing (clearly named)
pub struct MockStorageForTesting {
    test_double: StorageTestDouble,
    fake_data: Arc<Mutex<HashMap<String, String>>>,
}

impl MockStorageForTesting {
    pub fn new() -> Self {
        Self {
            test_double: StorageTestDouble::new(TestDoubleConfig::default()),
            fake_data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn fake_create_pool(&self, name: &str) -> Result<(), TestStorageError> {
        self.test_double
            .simulate_operation(&format!("create_pool:{}", name))
            .await?;

        if let Ok(mut data) = self.fake_data.lock() {
            data.insert(format!("pool:{}", name), "created".to_string());
        }

        Ok(())
    }

    pub async fn fake_create_dataset(
        &self,
        pool: &str,
        dataset: &str,
        _tier: StorageTier,
    ) -> Result<(), TestStorageError> {
        self.test_double
            .simulate_operation(&format!("create_dataset:{}:{}", pool, dataset))
            .await?;

        if let Ok(mut data) = self.fake_data.lock() {
            data.insert(
                format!("dataset:{}:{}", pool, dataset),
                "created".to_string(),
            );
        }

        Ok(())
    }

    pub fn get_test_operations(&self) -> Vec<String> {
        self.test_double.get_operations()
    }

    pub fn simulate_failure(&self, operation: &str) {
        self.test_double.add_failure(operation);
    }
}

/// Test-specific storage error
#[derive(Debug, thiserror::Error)]
pub enum TestStorageError {
    #[error("Simulated test failure for operation: {0}")]
    SimulatedFailure(String),

    #[error("Test configuration error: {0}")]
    ConfigError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_storage_test_double() -> Result<(), Box<dyn std::error::Error>> {
        let test_double = StorageTestDouble::new(TestDoubleConfig::default());

        let result = test_double.simulate_operation("test_operation").await;
        assert!(result.is_ok());

        let operations = test_double.get_operations();
        assert_eq!(operations.len(), 1);
        assert_eq!(operations[0], "test_operation");
    }

    #[tokio::test]
    async fn test_mock_storage_creation() -> Result<(), Box<dyn std::error::Error>> {
        let mock = MockStorageForTesting::new();

        let result = mock.fake_create_pool("test-pool").await;
        assert!(result.is_ok());

        let operations = mock.get_test_operations();
        assert!(operations.contains(&"create_pool:test-pool".to_string()));
    }

    #[tokio::test]
    async fn test_simulated_failures() -> Result<(), Box<dyn std::error::Error>> {
        let mock = MockStorageForTesting::new();
        mock.simulate_failure("create_pool:fail-pool");

        let result = mock.fake_create_pool("fail-pool").await;
        assert!(result.is_err());

        match result {
            Err(TestStorageError::SimulatedFailure(op)) => {
                assert_eq!(op, "create_pool:fail-pool");
            }
            _ => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Test assertion failed",
                )));
            }
        }
    }
}
