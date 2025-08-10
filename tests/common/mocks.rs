/// Clean mock implementations for testing
/// Avoids complex dependencies and focuses on what actually works
use super::{NestGateError, Result, UnifiedServiceType};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Simple mock service that implements basic functionality
#[derive(Debug, Clone)]
pub struct MockUniversalService {
    pub service_type: UnifiedServiceType,
    pub name: String,
    pub enabled: bool,
    pub call_count: Arc<Mutex<usize>>,
    pub last_request: Arc<Mutex<Option<String>>>,
}

impl MockUniversalService {
    pub fn new(service_type: UnifiedServiceType, name: String) -> Self {
        Self {
            service_type,
            name,
            enabled: true,
            call_count: Arc::new(Mutex::new(0)),
            last_request: Arc::new(Mutex::new(None)),
        }
    }

    pub fn storage(name: &str) -> Self {
        Self::new(UnifiedServiceType::Storage, name.to_string())
    }

    pub fn network(name: &str) -> Self {
        Self::new(UnifiedServiceType::Network, name.to_string())
    }

    pub fn security(name: &str) -> Self {
        Self::new(UnifiedServiceType::Security, name.to_string())
    }

    pub fn compute(name: &str) -> Self {
        Self::new(UnifiedServiceType::Compute, name.to_string())
    }

    /// Simulate handling a request
    pub async fn handle_request(&self, request: &str) -> Result<String> {
        // Update call count
        if let Ok(mut count) = self.call_count.lock() {
            *count += 1;
        }

        // Store last request
        if let Ok(mut last) = self.last_request.lock() {
            *last = Some(request.to_string());
        }

        if !self.enabled {
            return Err(NestGateError::internal_error(
                format!("Service {} is disabled", self.name),
                "mock_service".to_string(),
            ));
        }

        // Simulate processing time
        tokio::time::sleep(Duration::from_millis(1)).await;

        Ok(format!(
            "Response from {} service for request: {}",
            self.name, request
        ))
    }

    /// Get call count
    pub fn get_call_count(&self) -> usize {
        self.call_count.lock().map(|count| *count).unwrap_or(0)
    }

    /// Get last request
    pub fn get_last_request(&self) -> Option<String> {
        self.last_request.lock().ok()?.clone()
    }

    /// Enable/disable the service
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

/// Mock storage service with basic operations
#[derive(Debug)]
pub struct MockStorageService {
    pub name: String,
    pub data: Arc<RwLock<HashMap<String, Vec<u8>>>>,
    pub operations: Arc<Mutex<usize>>,
}

impl MockStorageService {
    pub fn new(name: String) -> Self {
        Self {
            name,
            data: Arc::new(RwLock::new(HashMap::new())),
            operations: Arc::new(Mutex::new(0)),
        }
    }

    pub async fn store(&self, key: String, value: Vec<u8>) -> Result<()> {
        self.increment_operations();

        let mut data = self.data.write().await;
        data.insert(key, value);

        // Simulate storage delay
        tokio::time::sleep(Duration::from_millis(2)).await;
        Ok(())
    }

    pub async fn retrieve(&self, key: &str) -> Result<Option<Vec<u8>>> {
        self.increment_operations();

        let data = self.data.read().await;
        let result = data.get(key).cloned();

        // Simulate retrieval delay
        tokio::time::sleep(Duration::from_millis(1)).await;
        Ok(result)
    }

    pub async fn delete(&self, key: &str) -> Result<bool> {
        self.increment_operations();

        let mut data = self.data.write().await;
        let existed = data.remove(key).is_some();

        // Simulate deletion delay
        tokio::time::sleep(Duration::from_millis(1)).await;
        Ok(existed)
    }

    pub async fn list_keys(&self) -> Result<Vec<String>> {
        self.increment_operations();

        let data = self.data.read().await;
        let keys = data.keys().cloned().collect();

        Ok(keys)
    }

    pub fn get_operation_count(&self) -> usize {
        self.operations.lock().map(|ops| *ops).unwrap_or(0)
    }

    fn increment_operations(&self) {
        if let Ok(mut ops) = self.operations.lock() {
            *ops += 1;
        }
    }
}

/// Mock network service for testing
#[derive(Debug)]
pub struct MockNetworkService {
    pub name: String,
    pub connections: Arc<Mutex<Vec<MockConnection>>>,
    pub total_bytes_sent: Arc<Mutex<usize>>,
    pub total_bytes_received: Arc<Mutex<usize>>,
}

#[derive(Debug, Clone)]
pub struct MockConnection {
    pub id: String,
    pub connected_at: Instant,
    pub bytes_sent: usize,
    pub bytes_received: usize,
}

impl MockNetworkService {
    pub fn new(name: String) -> Self {
        Self {
            name,
            connections: Arc::new(Mutex::new(Vec::new())),
            total_bytes_sent: Arc::new(Mutex::new(0)),
            total_bytes_received: Arc::new(Mutex::new(0)),
        }
    }

    pub async fn create_connection(&self, connection_id: String) -> Result<()> {
        let connection = MockConnection {
            id: connection_id,
            connected_at: Instant::now(),
            bytes_sent: 0,
            bytes_received: 0,
        };

        if let Ok(mut connections) = self.connections.lock() {
            connections.push(connection);
        }

        // Simulate connection setup
        tokio::time::sleep(Duration::from_millis(5)).await;
        Ok(())
    }

    pub async fn send_data(&self, connection_id: &str, data: &[u8]) -> Result<()> {
        if let Ok(mut connections) = self.connections.lock() {
            if let Some(conn) = connections.iter_mut().find(|c| c.id == connection_id) {
                conn.bytes_sent += data.len();
            }
        }

        if let Ok(mut total) = self.total_bytes_sent.lock() {
            *total += data.len();
        }

        // Simulate network delay
        tokio::time::sleep(Duration::from_millis(1)).await;
        Ok(())
    }

    pub fn get_connection_count(&self) -> usize {
        self.connections.lock().map(|c| c.len()).unwrap_or(0)
    }

    pub fn get_total_bytes_sent(&self) -> usize {
        self.total_bytes_sent.lock().map(|b| *b).unwrap_or(0)
    }
}

/// Mock registry for managing multiple services
#[derive(Debug, Clone)]
pub struct MockServiceRegistry {
    pub services: Arc<RwLock<HashMap<String, MockUniversalService>>>,
    pub storage_services: Arc<RwLock<HashMap<String, MockStorageService>>>,
    pub network_services: Arc<RwLock<HashMap<String, MockNetworkService>>>,
}

impl MockServiceRegistry {
    pub fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
            storage_services: Arc::new(RwLock::new(HashMap::new())),
            network_services: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register_universal_service(&self, service: MockUniversalService) {
        let mut services = self.services.write().await;
        services.insert(service.name.clone(), service);
    }

    pub async fn register_storage_service(&self, service: MockStorageService) {
        let mut services = self.storage_services.write().await;
        services.insert(service.name.clone(), service);
    }

    pub async fn register_network_service(&self, service: MockNetworkService) {
        let mut services = self.network_services.write().await;
        services.insert(service.name.clone(), service);
    }

    pub async fn get_universal_service(&self, name: &str) -> Option<MockUniversalService> {
        let services = self.services.read().await;
        services.get(name).cloned()
    }

    pub async fn list_service_names(&self) -> Vec<String> {
        let services = self.services.read().await;
        services.keys().cloned().collect()
    }

    pub async fn get_service_count(&self) -> usize {
        let services = self.services.read().await;
        services.len()
    }
}

impl Default for MockServiceRegistry {
    fn default() -> Self {
        Self::new()
    }
}
