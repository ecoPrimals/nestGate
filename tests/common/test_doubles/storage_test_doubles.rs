// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Storage Test Doubles
//!
//! Pure test mocks for testing storage functionality.
//! They simulate various failure conditions and edge cases.
//! These are test infrastructure only — not hardware abstractions.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
// Note: sleep and Duration available if needed for storage simulation

use super::TestDoubleConfig;
// Use canonical types - the modern location
use nestgate_core::canonical_types::storage::StorageTier;

/// Storage test double for unit testing
///
/// This is a pure test mock that simulates storage operations for testing purposes.
/// It can be configured to simulate various failure conditions.
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
        Self {
            config: TestDoubleConfig {
                simulate_failures: true,
                ..TestDoubleConfig::default()
            },
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
        self.operations
            .lock()
            .map(|ops| ops.clone())
            .unwrap_or_default()
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

impl Default for MockStorageForTesting {
    fn default() -> Self {
        Self::new()
    }
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

    /// Initialize the mock storage
    pub fn initialize(&mut self) -> Result<(), TestStorageError> {
        Ok(())
    }

    /// Cleanup the mock storage
    pub fn cleanup(&mut self) -> Result<(), String> {
        self.reset()
    }

    /// Reset the mock storage to initial state
    pub fn reset(&mut self) -> Result<(), String> {
        if let Ok(mut ops) = self.test_double.operations.lock() {
            ops.clear();
        }
        if let Ok(mut failures) = self.test_double.should_fail_operations.lock() {
            failures.clear();
        }
        Ok(())
    }
}

/// Test-specific storage error - simple implementation for tests
#[derive(Debug)]
pub enum TestStorageError {
    SimulatedFailure(String),
    ConfigError(String),
}

impl std::fmt::Display for TestStorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SimulatedFailure(op) => write!(f, "Simulated test failure for operation: {}", op),
            Self::ConfigError(msg) => write!(f, "Test configuration error: {}", msg),
        }
    }
}

impl std::error::Error for TestStorageError {}

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

        Ok(())
    }

    #[tokio::test]
    async fn test_mock_storage_creation() -> Result<(), Box<dyn std::error::Error>> {
        let mock = MockStorageForTesting::new();

        let result = mock.fake_create_pool("test-pool").await;
        assert!(result.is_ok());

        let operations = mock.get_test_operations();
        assert!(operations.contains(&"create_pool:test-pool".to_string()));

        Ok(())
    }

    #[tokio::test]
    async fn test_simulated_failures() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mock = MockStorageForTesting::new();
        mock.simulate_failure("create_pool:fail-pool");

        let result = mock.fake_create_pool("fail-pool").await;
        assert!(result.is_err());

        match result {
            Err(TestStorageError::SimulatedFailure(op)) => {
                assert_eq!(op, "create_pool:fail-pool");
            }
            _ => {
                return Err(Box::new(std::io::Error::other("Test assertion failed"))
                    as Box<dyn std::error::Error + Send + Sync>);
            }
        }

        Ok(())
    }
}
