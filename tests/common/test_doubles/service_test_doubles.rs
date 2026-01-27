//! Service Test Doubles
//!
//! Pure test mocks for service functionality testing.
//! These simulate service operations, registrations, and failures.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::TestDoubleConfig;

/// Service test double for testing service operations
pub struct ServiceTestDouble {
    #[allow(dead_code)] // Test fixture field
    config: TestDoubleConfig,
    registered_services: Arc<Mutex<HashMap<String, TestService>>>,
    operations: Arc<Mutex<Vec<String>>>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)] // Test fixture
struct TestService {
    name: String,
    status: ServiceStatus,
    endpoint: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)] // Test fixture
enum ServiceStatus {
    Running,
    Stopped,
    Failed,
}

impl ServiceTestDouble {
    pub fn new(config: TestDoubleConfig) -> Self {
        Self {
            config,
            registered_services: Arc::new(Mutex::new(HashMap::new())),
            operations: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn fake_register_service(
        &self,
        name: &str,
        endpoint: &str,
    ) -> Result<(), ServiceTestError> {
        self.record_operation(&format!("register_service:{}", name))
            .await?;

        if let Ok(mut services) = self.registered_services.lock() {
            services.insert(
                name.to_string(),
                TestService {
                    name: name.to_string(),
                    status: ServiceStatus::Running,
                    endpoint: endpoint.to_string(),
                },
            );
        }

        Ok(())
    }

    pub fn get_operations(&self) -> Vec<String> {
        self.operations
            .lock()
            .map(|ops| ops.clone())
            .unwrap_or_default()
    }

    async fn record_operation(&self, operation: &str) -> Result<(), ServiceTestError> {
        if let Ok(mut ops) = self.operations.lock() {
            ops.push(operation.to_string());
        }
        Ok(())
    }
}

/// Mock service for testing
pub struct MockServiceForTesting {
    test_double: ServiceTestDouble,
}

impl Default for MockServiceForTesting {
    fn default() -> Self {
        Self::new()
    }
}

impl MockServiceForTesting {
    pub fn new() -> Self {
        Self {
            test_double: ServiceTestDouble::new(TestDoubleConfig::default()),
        }
    }

    pub async fn fake_start_service(&self, name: &str) -> Result<(), ServiceTestError> {
        self.test_double
            .record_operation(&format!("start_service:{}", name))
            .await
    }

    /// Initialize the mock service
    pub fn initialize(&mut self) -> Result<(), ServiceTestError> {
        Ok(())
    }

    /// Cleanup the mock service
    pub fn cleanup(&mut self) -> Result<(), String> {
        self.reset()
    }

    /// Reset the mock service to initial state
    pub fn reset(&mut self) -> Result<(), String> {
        if let Ok(mut ops) = self.test_double.operations.lock() {
            ops.clear();
        }
        Ok(())
    }
}

// Simple error type for test doubles - no thiserror needed in tests
#[derive(Debug)]
pub enum ServiceTestError {
    SimulatedFailure(String),
}

impl std::fmt::Display for ServiceTestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SimulatedFailure(msg) => write!(f, "Simulated service failure: {}", msg),
        }
    }
}

impl std::error::Error for ServiceTestError {}
