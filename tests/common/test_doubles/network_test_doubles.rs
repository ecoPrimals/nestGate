// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Network Test Doubles
//!
//! Pure test mocks for network functionality testing.
//! These simulate network operations, failures, and latency for unit testing.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
// Note: sleep and Duration available if needed for network simulation

use super::TestDoubleConfig;

/// Network test double for testing network operations
pub struct NetworkTestDouble {
    #[expect(dead_code)] // Test fixture field
    config: TestDoubleConfig,
    connections: Arc<Mutex<HashMap<String, ConnectionStatus>>>,
    operations: Arc<Mutex<Vec<String>>>,
}

#[derive(Debug, Clone)]
#[expect(dead_code)] // Test fixture
enum ConnectionStatus {
    Connected,
    Disconnected,
    Failed,
}

impl NetworkTestDouble {
    pub fn new(config: TestDoubleConfig) -> Self {
        Self {
            config,
            connections: Arc::new(Mutex::new(HashMap::new())),
            operations: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn simulate_connection(&self, endpoint: &str) -> Result<(), NetworkTestError> {
        self.record_operation(&format!("connect:{}", endpoint))
            .await?;

        if let Ok(mut connections) = self.connections.lock() {
            connections.insert(endpoint.to_string(), ConnectionStatus::Connected);
        }

        Ok(())
    }

    pub async fn simulate_disconnection(&self, endpoint: &str) -> Result<(), NetworkTestError> {
        self.record_operation(&format!("disconnect:{}", endpoint))
            .await?;

        if let Ok(mut connections) = self.connections.lock() {
            connections.insert(endpoint.to_string(), ConnectionStatus::Disconnected);
        }

        Ok(())
    }

    pub fn get_operations(&self) -> Vec<String> {
        self.operations
            .lock()
            .map(|ops| ops.clone())
            .unwrap_or_default()
    }

    async fn record_operation(&self, operation: &str) -> Result<(), NetworkTestError> {
        if let Ok(mut ops) = self.operations.lock() {
            ops.push(operation.to_string());
        }

        if self.config.response_delay_ms > 0 {
            tokio::task::yield_now().await;
        }

        Ok(())
    }
}

/// Mock network service for testing
pub struct MockNetworkForTesting {
    test_double: NetworkTestDouble,
}

impl Default for MockNetworkForTesting {
    fn default() -> Self {
        Self::new()
    }
}

impl MockNetworkForTesting {
    pub fn new() -> Self {
        Self {
            test_double: NetworkTestDouble::new(TestDoubleConfig::default()),
        }
    }

    pub async fn fake_send_request(
        &self,
        endpoint: &str,
        _data: &str,
    ) -> Result<String, NetworkTestError> {
        self.test_double
            .record_operation(&format!("send_request:{}", endpoint))
            .await?;
        Ok("fake_response".to_string())
    }

    /// Initialize the mock network
    pub fn initialize(&mut self) -> Result<(), NetworkTestError> {
        Ok(())
    }

    /// Cleanup the mock network
    pub fn cleanup(&mut self) -> Result<(), String> {
        self.reset()
    }

    /// Reset the mock network to initial state
    pub fn reset(&mut self) -> Result<(), String> {
        if let Ok(mut ops) = self.test_double.operations.lock() {
            ops.clear();
        }
        Ok(())
    }
}

// Simple error type for test doubles - no thiserror needed in tests
#[derive(Debug)]
pub enum NetworkTestError {
    SimulatedFailure(String),
    Timeout,
}

impl std::fmt::Display for NetworkTestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SimulatedFailure(msg) => write!(f, "Simulated network failure: {}", msg),
            Self::Timeout => write!(f, "Test connection timeout"),
        }
    }
}

impl std::error::Error for NetworkTestError {}
