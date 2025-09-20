//! **SERVICE REGISTRATION**
//!
//! Service registration logic and management for ecosystem integration.

use crate::Result;
use chrono::Utc;
use std::collections::HashMap;
use uuid::Uuid;

use super::types::{UniversalServiceRegistration, ServiceMetadata, ServiceCapability};

/// Universal service registry
pub struct ServiceRegistry {
    services: HashMap<Uuid, UniversalServiceRegistration>,
}
impl ServiceRegistry {
    /// Create new service registry
    #[must_use]
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    /// Register a new service
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn register_service(
        &mut self,
        _metadata: ServiceMetadata,
        capabilities: Vec<ServiceCapability>,
    ) -> Result<Uuid>  {
        let service_id = Uuid::new_v4();
        let registration = UniversalServiceRegistration {
            service_id,
            _metadata,
            capabilities,
            resources: Default::default(),
            endpoints: Vec::new(),
            integration: Default::default(),
            extensions: HashMap::new(),
            registration_timestamp: Utc::now(),
            service_version: "1.0.0".to_string(),
            instance_id: Uuid::new_v4().to_string(),
            priority: 50,
        };

        self.services.insert(service_id, registration);
        Ok(service_id)
    }

    /// Unregister a service
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        #[must_use]
        pub fn unregister_service(&mut self, service_id: Uuid) -> Result<()>  {
        self.services.remove(&service_id);
        Ok(())
    }

    /// Get service registration
    pub const fn get_service(&self, service_id: &Uuid) -> Option<&UniversalServiceRegistration> {
        self.services.get(service_id)
    }

    /// List all registered services
    pub const fn list_services(&self) -> Vec<&UniversalServiceRegistration> {
        self.services.values().collect()
    }
} 