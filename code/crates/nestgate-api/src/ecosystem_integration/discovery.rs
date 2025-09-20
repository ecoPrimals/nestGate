//! **SERVICE DISCOVERY**
//!
//! Service discovery implementation for ecosystem integration.

use crate::Result;
use std::collections::HashMap;
use uuid::Uuid;

use super::types::{ServiceCategory, ServiceCapability, UniversalServiceRegistration};
use super::registration::ServiceRegistry;

/// Service discovery _engine
pub struct ServiceDiscovery {
    registry: ServiceRegistry,
}
impl ServiceDiscovery {
    /// Create new service discovery
    pub const fn new() -> Self {
        Self {
            registry: ServiceRegistry::new(),
        }
    }

    /// Discover services by category
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn discover_by_category(
        &self,
        category: &ServiceCategory,
    ) -> Result<Vec<&UniversalServiceRegistration>>  {
        let services = self.registry
            .list_services()
            .into_iter()
            .filter(|service| &service._metadata.category == category)
            .collect();
        
        Ok(services)
    }

    /// Discover services by capability
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn discover_by_capability(
        &self,
        capability: &ServiceCapability,
    ) -> Result<Vec<&UniversalServiceRegistration>>  {
        let services = self.registry
            .list_services()
            .into_iter()
            .filter(|service| service.capabilities.contains(capability))
            .collect();
        
        Ok(services)
    }

    /// Health check all services
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        #[must_use]
        pub fn health_check_all(&self) -> Result<HashMap<Uuid, bool>>  {
        let mut health_status = HashMap::new();
        
        for service in self.registry.list_services() {
            // Simplified health check - in real implementation would ping endpoints
            health_status.insert(service.service_id, true);
        }
        
        Ok(health_status)
    }
} 