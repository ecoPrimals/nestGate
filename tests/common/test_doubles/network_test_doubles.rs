//! Network Test Doubles
//!
//! Pure test mocks for network functionality testing.
//! These simulate network operations, failures, and latency for unit testing.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

use super::TestDoubleConfig;

/// Network test double for testing network operations
pub struct NetworkTestDouble {
    config: TestDoubleConfig,
    connections: Arc<Mutex<HashMap<String, ConnectionStatus>>>,
    operations: Arc<Mutex<Vec<String>>>,
}

#[derive(Debug, Clone)]
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
        self.record_operation(&format!("connect:{}", endpoint)).await?;
        
        if let Ok(mut connections) = self.connections.lock() {
            connections.insert(endpoint.to_string(), ConnectionStatus::Connected);
        }
        
        Ok(())
    }
    
    pub async fn simulate_disconnection(&self, endpoint: &str) -> Result<(), NetworkTestError> {
        self.record_operation(&format!("disconnect:{}", endpoint)).await?;
        
        if let Ok(mut connections) = self.connections.lock() {
            connections.insert(endpoint.to_string(), ConnectionStatus::Disconnected);
        }
        
        Ok(())
    }
    
    pub fn get_operations(&self) -> Vec<String> {
        self.operations.lock().unwrap().clone()
    }
    
    async fn record_operation(&self, operation: &str) -> Result<(), NetworkTestError> {
        if let Ok(mut ops) = self.operations.lock() {
            ops.push(operation.to_string());
        }
        
        if self.config.response_delay_ms > 0 {
            sleep(Duration::from_millis(self.config.response_delay_ms)).await;
        }
        
        Ok(())
    }
}

/// Mock network service for testing
pub struct MockNetworkForTesting {
    test_double: NetworkTestDouble,
}

impl MockNetworkForTesting {
    pub fn new() -> Self {
        Self {
            test_double: NetworkTestDouble::new(TestDoubleConfig::default()),
        }
    }
    
    pub async fn fake_send_request(&self, endpoint: &str, _data: &str) -> Result<String, NetworkTestError> {
        self.test_double.record_operation(&format!("send_request:{}", endpoint)).await?;
        Ok("fake_response".to_string())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum NetworkTestError {
    #[error("Simulated network failure: {0}")]
    SimulatedFailure(String),
    
    #[error("Test connection timeout")]
    Timeout,
} 