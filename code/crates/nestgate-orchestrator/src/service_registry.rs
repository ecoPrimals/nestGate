//! Service registry for managing local services

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use nestgate_core::Result;

/// Service registry for managing service endpoints and discovery
#[derive(Debug)]
pub struct ServiceRegistry {
    /// Registered services
    services: Arc<RwLock<HashMap<String, ServiceInfo>>>,
    /// Running state
    running: Arc<RwLock<bool>>,
}

/// Information about a registered service
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServiceInfo {
    /// Service name
    pub name: String,
    /// Service endpoint (e.g., "http://localhost:8081")
    pub endpoint: String,
    /// Service health status
    pub health_status: HealthStatus,
    /// Last health check timestamp
    pub last_health_check: Option<chrono::DateTime<chrono::Utc>>,
    /// Service metadata
    pub metadata: HashMap<String, String>,
}

/// Health status of a service
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum HealthStatus {
    /// Service is healthy
    Healthy,
    /// Service is unhealthy
    Unhealthy,
    /// Service health is unknown
    Unknown,
}

impl ServiceRegistry {
    /// Create a new service registry
    pub fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
            running: Arc::new(RwLock::new(false)),
        }
    }
    
    /// Start the service registry
    pub async fn start(&self) -> Result<()> {
        tracing::debug!("Starting service registry");
        
        let mut running = self.running.write().await;
        *running = true;
        
        tracing::debug!("Service registry started");
        Ok(())
    }
    
    /// Stop the service registry
    pub async fn stop(&self) -> Result<()> {
        tracing::debug!("Stopping service registry");
        
        let mut running = self.running.write().await;
        *running = false;
        
        // Clear all services
        let mut services = self.services.write().await;
        services.clear();
        
        tracing::debug!("Service registry stopped");
        Ok(())
    }
    
    /// Register a service
    pub async fn register_service(&self, service_info: ServiceInfo) -> Result<()> {
        tracing::debug!("Registering service: {}", service_info.name);
        
        let mut services = self.services.write().await;
        services.insert(service_info.name.clone(), service_info);
        
        Ok(())
    }
    
    /// Unregister a service
    pub async fn unregister_service(&self, service_name: &str) -> Result<()> {
        tracing::debug!("Unregistering service: {}", service_name);
        
        let mut services = self.services.write().await;
        services.remove(service_name);
        
        Ok(())
    }
    
    /// Get a service endpoint
    pub async fn get_service_endpoint(&self, service_name: &str) -> Result<String> {
        let services = self.services.read().await;
        
        match services.get(service_name) {
            Some(service_info) => Ok(service_info.endpoint.clone()),
            None => Err(nestgate_core::NestGateError::NotFound(
                format!("Service not found: {}", service_name)
            )),
        }
    }
    
    /// Get service information
    pub async fn get_service_info(&self, service_name: &str) -> Result<ServiceInfo> {
        let services = self.services.read().await;
        
        match services.get(service_name) {
            Some(service_info) => Ok(service_info.clone()),
            None => Err(nestgate_core::NestGateError::NotFound(
                format!("Service not found: {}", service_name)
            )),
        }
    }
    
    /// List all registered services
    pub async fn list_services(&self) -> Result<Vec<ServiceInfo>> {
        let services = self.services.read().await;
        Ok(services.values().cloned().collect())
    }
    
    /// Update service health status
    pub async fn update_service_health(&self, service_name: &str, status: HealthStatus) -> Result<()> {
        let mut services = self.services.write().await;
        
        if let Some(service_info) = services.get_mut(service_name) {
            service_info.health_status = status;
            service_info.last_health_check = Some(chrono::Utc::now());
        }
        
        Ok(())
    }
    
    /// Get healthy services only
    pub async fn get_healthy_services(&self) -> Result<Vec<ServiceInfo>> {
        let services = self.services.read().await;
        
        let healthy_services: Vec<ServiceInfo> = services
            .values()
            .filter(|service| matches!(service.health_status, HealthStatus::Healthy))
            .cloned()
            .collect();
            
        Ok(healthy_services)
    }
} 