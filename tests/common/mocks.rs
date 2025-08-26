//! Mock implementations for testing

use crate::canonical_modernization::{UnifiedServiceType, Result};
use std::collections::HashMap;

/// Mock service registry
#[derive(Debug, Clone)]
pub struct MockServiceRegistry {
    services: HashMap<String, SimpleTestService>,
}

impl MockServiceRegistry {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    pub fn register_service(&mut self, name: String, service: SimpleTestService) {
        self.services.insert(name, service);
    }

    pub fn get_service(&self, name: &str) -> Option<&SimpleTestService> {
        self.services.get(name)
    }

    pub fn list_services(&self) -> Vec<String> {
        self.services.keys().cloned().collect()
    }

    // Additional methods needed by tests
    pub async fn register_universal_service(&mut self, service: MockUniversalService) {
        let simple_service = SimpleTestService::new(service.name);
        let service_name = simple_service.name.clone();
        self.services.insert(service_name, simple_service);
    }

    pub async fn get_universal_service(&self, name: &str) -> Option<SimpleTestService> {
        self.services.get(name).cloned()
    }

    pub async fn list_service_names(&self) -> Vec<String> {
        self.list_services()
    }

    pub async fn get_service_count(&self) -> usize {
        self.services.len()
    }
}

impl Default for MockServiceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Mock storage service
#[derive(Debug, Clone)]
pub struct MockStorageService {
    pub name: String,
    pub capacity: u64,
    pub used: u64,
}

impl MockStorageService {
    pub fn new(name: String) -> Self {
        Self {
            name,
            capacity: 1024 * 1024 * 1024, // 1GB
            used: 0,
        }
    }

    pub async fn store_data(&mut self, data: Vec<u8>) -> Result<String> {
        self.used += data.len() as u64;
        Ok(format!("stored_{}_bytes", data.len()))
    }

    pub async fn retrieve_data(&self, _id: &str) -> Result<Vec<u8>> {
        Ok(vec![0u8; 1024]) // Mock data
    }

    // Additional methods needed by tests
    pub async fn store(&mut self, _key: String, value: Vec<u8>) -> Result<()> {
        self.used += value.len() as u64;
        // Mock implementation - in real scenarios this would track operations
        Ok(())
    }

    pub async fn retrieve(&self, _key: &str) -> Result<Option<Vec<u8>>> {
        Ok(Some(vec![0u8; 1024])) // Mock data
    }

    pub fn get_operation_count(&self) -> usize {
        // Mock implementation - in real scenarios this would track operations
        1
    }
}

/// Mock universal service
#[derive(Debug, Clone)]
pub struct MockUniversalService {
    pub service_type: UnifiedServiceType,
    pub name: String,
    pub status: ServiceStatus,
}

#[derive(Debug, Clone)]
pub enum ServiceStatus {
    Running,
    Stopped,
    Error(String),
}

impl MockUniversalService {
    pub fn new(service_type: UnifiedServiceType, name: String) -> Self {
        Self {
            service_type,
            name,
            status: ServiceStatus::Running,
        }
    }

    pub fn storage(name: &str) -> Self {
        Self::new(UnifiedServiceType::Storage, name.to_string())
    }

    pub fn network(name: &str) -> Self {
        Self::new(UnifiedServiceType::Network, name.to_string())
    }

    pub fn compute(name: &str) -> Self {
        Self::new(UnifiedServiceType::Compute, name.to_string())
    }

    pub async fn start(&mut self) -> Result<()> {
        self.status = ServiceStatus::Running;
        Ok(())
    }

    pub async fn stop(&mut self) -> Result<()> {
        self.status = ServiceStatus::Stopped;
        Ok(())
    }

    pub fn is_running(&self) -> bool {
        matches!(self.status, ServiceStatus::Running)
    }

    // Additional methods needed by tests
    pub async fn handle_request(&self, request: &str) -> Result<String> {
        if !self.is_running() {
            return Err(nestgate_core::NestGateError::internal_error(
                format!("Service {} is not running", self.name),
                "mock_service".to_string(),
            ));
        }
        Ok(format!("Response from {} for: {}", self.name, request))
    }

    pub fn get_call_count(&self) -> usize {
        // Mock implementation - in real scenarios this would track calls
        1
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        if enabled {
            self.status = ServiceStatus::Running;
        } else {
            self.status = ServiceStatus::Stopped;
        }
    }
}
