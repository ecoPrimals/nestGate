//! Service Registry
//! 
//! Enhanced service registry with advanced integration service management
//! with v2 orchestrator-centric architecture

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug, warn};

use nestgate_mcp::{
    protocol::ServiceInfo,
    error::{Result, Error},
};

/// Enhanced Service Registry with enhanced NestGate capabilities
pub struct ServiceRegistry {
    services: Arc<RwLock<HashMap<String, ServiceInfo>>>,
    service_types: Arc<RwLock<HashMap<String, Vec<String>>>>, // service_type -> service_ids
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
            service_types: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a service
    pub async fn register(&self, service_info: ServiceInfo) -> Result<()> {
        let service_id = service_info.service_id.clone();
        let service_type = service_info.service_type.clone();
        
        info!("Registering service: {} (type: {})", service_id, service_type);
        
        // Store service info
        {
            let mut services = self.services.write().await;
            services.insert(service_id.clone(), service_info);
        }
        
        // Update service type index
        {
            let mut service_types = self.service_types.write().await;
            service_types
                .entry(service_type)
                .or_insert_with(Vec::new)
                .push(service_id);
        }
        
        debug!("Service registered successfully");
        Ok(())
    }

    /// Unregister a service
    pub async fn unregister(&self, service_id: &str) -> Result<()> {
        info!("Unregistering service: {}", service_id);
        
        let service_type = {
            let mut services = self.services.write().await;
            if let Some(service_info) = services.remove(service_id) {
                service_info.service_type
            } else {
                warn!("Attempted to unregister unknown service: {}", service_id);
                return Err(Error::validation(format!("Service not found: {}", service_id)));
            }
        };
        
        // Update service type index
        {
            let mut service_types = self.service_types.write().await;
            if let Some(service_ids) = service_types.get_mut(&service_type) {
                service_ids.retain(|id| id != service_id);
                if service_ids.is_empty() {
                    service_types.remove(&service_type);
                }
            }
        }
        
        debug!("Service unregistered successfully");
        Ok(())
    }

    /// Get service by ID
    pub async fn get_service(&self, service_id: &str) -> Result<Option<ServiceInfo>> {
        let services = self.services.read().await;
        Ok(services.get(service_id).cloned())
    }

    /// List all services
    pub async fn list_services(&self) -> Result<Vec<ServiceInfo>> {
        let services = self.services.read().await;
        Ok(services.values().cloned().collect())
    }

    /// List services by type
    pub async fn list_services_by_type(&self, service_type: &str) -> Result<Vec<ServiceInfo>> {
        let service_types = self.service_types.read().await;
        let services = self.services.read().await;
        
        if let Some(service_ids) = service_types.get(service_type) {
            let mut result = Vec::new();
            for service_id in service_ids {
                if let Some(service_info) = services.get(service_id) {
                    result.push(service_info.clone());
                }
            }
            Ok(result)
        } else {
            Ok(Vec::new())
        }
    }

    /// Get service count
    pub async fn service_count(&self) -> usize {
        let services = self.services.read().await;
        services.len()
    }

    /// Check if service exists
    pub async fn exists(&self, service_id: &str) -> bool {
        let services = self.services.read().await;
        services.contains_key(service_id)
    }
}

impl Default for ServiceRegistry {
    fn default() -> Self {
        Self::new()
    }
} 