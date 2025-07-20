//! Universal service discovery module

use crate::{Result, ServiceInstance, ServiceStatus};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// Discovered service information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredService {
    pub name: String,
    pub service_type: String,
    pub endpoint: String,
    pub capabilities: Vec<String>,
    pub discovered_via: String,
    pub last_seen: SystemTime,
}

/// Universal service discovery manager
pub struct ServiceDiscovery {
    discovered_services: HashMap<String, DiscoveredService>,
}

impl ServiceDiscovery {
    /// Create a new service discovery instance
    pub fn new() -> Self {
        Self {
            discovered_services: HashMap::new(),
        }
    }

    /// Discover services by type
    pub async fn discover_services(&self, service_type: &str) -> Result<Vec<ServiceInstance>> {
        let services: Vec<ServiceInstance> = self
            .discovered_services
            .values()
            .filter(|service| service.service_type == service_type)
            .map(|discovered| ServiceInstance {
                id: discovered.name.clone(),
                name: discovered.name.clone(),
                service_type: discovered.service_type.clone(),
                address: discovered.endpoint.clone(),
                port: 8080, // Default port
                host: format!("{}:8080", discovered.endpoint),
                status: ServiceStatus::Unknown,
                metadata: HashMap::new(),
                last_seen: discovered.last_seen,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            })
            .collect();

        Ok(services)
    }

    /// Add a discovered service
    pub fn add_service(&mut self, service: DiscoveredService) {
        self.discovered_services
            .insert(service.name.clone(), service);
    }

    /// Get all discovered services
    pub fn get_all_services(&self) -> Vec<DiscoveredService> {
        self.discovered_services.values().cloned().collect()
    }
}

impl Default for ServiceDiscovery {
    fn default() -> Self {
        Self::new()
    }
}
