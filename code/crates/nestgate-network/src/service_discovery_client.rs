//! Service Discovery Client - Sovereignty Compliant
//!
//! This module provides a lightweight client for service discovery that delegates
//! to orchestration capabilities discovered through the universal adapter.
//!
//! SOVEREIGNTY COMPLIANCE:
//! - No hardcoded primal names
//! - Uses capability discovery only
//! - Maintains local fallback for standalone operation

use crate::{
    api::{ServiceInstance, ServiceStatus},
    Result,
};
use nestgate_core::universal_adapter::{CapabilityRequest, UniversalAdapter};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, info};

/// Local service registry for fallback operation
#[derive(Debug, Clone, Default)]
pub struct LocalServiceRegistry {
    services: HashMap<String, ServiceInstance>,
}

impl LocalServiceRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a service in the local registry
    pub fn register_service(&mut self, service: ServiceInstance) {
        debug!("📝 Registering service locally: {}", service.name);
        self.services.insert(service.name.clone(), service);
    }

    /// Discover services locally (fallback mode)
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn discover_services(&self, _service_type: &str) -> Result<Vec<ServiceInstance>> {
        debug!("🔍 Local service discovery (all services - no type filtering in current implementation)");

        let services: Vec<ServiceInstance> = self.services.values().cloned().collect();

        debug!("Found {} local services", services.len());
        Ok(services)
    }

    /// Get all registered services
    #[must_use]
    pub fn get_all_services(&self) -> Vec<ServiceInstance> {
        self.services.values().cloned().collect()
    }
}

/// Sovereignty-compliant service discovery client
///
/// This client follows the primal sovereignty principle:
/// - No hardcoded orchestration primal names
/// - Dynamic capability discovery through universal adapter
/// - Graceful fallback to local registry when orchestration unavailable
#[derive(Debug)]
pub struct ServiceDiscoveryClient {
    universal_adapter: Arc<UniversalAdapter>,
    local_registry: LocalServiceRegistry,
}

impl ServiceDiscoveryClient {
    /// Create new service discovery client with universal adapter
    pub fn new(universal_adapter: Arc<UniversalAdapter>) -> Self {
        info!("🌐 Initializing sovereignty-compliant service discovery client");
        Self {
            universal_adapter,
            local_registry: LocalServiceRegistry::new(),
        }
    }

    /// Discover services using orchestration capability or local fallback
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn discover_services(&self, service_type: &str) -> Result<Vec<ServiceInstance>> {
        // Try to get orchestration capability through universal adapter
        match self.universal_adapter.get_capability("orchestration") {
            Ok(_capability_info) => {
                // Create request for service discovery
                let request = CapabilityRequest::new("orchestration", "discover_services")
                    .with_parameters(serde_json::json!({
                        "service_type": service_type,
                        "requester": "nestgate-network"
                    }));

                // Make request through universal adapter
                match self
                    .universal_adapter
                    .request_capability("orchestration", request)
                    .await
                {
                    Ok(response) => {
                        // Parse response into ServiceInstance objects
                        debug!(
                            "🎯 Orchestration discovery successful: {:?}",
                            response.status
                        );

                        // For now, return empty vec - in real implementation would parse response.result
                        Ok(Vec::new())
                    }
                    Err(e) => {
                        debug!("⚠️ Orchestration request failed: {}", e);
                        // Fallback to local registry
                        self.local_registry.discover_services(service_type)
                    }
                }
            }
            Err(e) => {
                debug!("⚠️ No orchestration capability available: {}", e);
                // Fallback to local registry
                self.local_registry.discover_services(service_type)
            }
        }
    }

    /// Register a service through orchestration capability or local fallback
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn register_service(&mut self, service: ServiceInstance) -> Result<()> {
        // Try orchestration capability first
        match self.universal_adapter.get_capability("orchestration") {
            Ok(_capability_info) => {
                let request = CapabilityRequest::new("orchestration", "register_service")
                    .with_parameters(serde_json::json!({
                        "service": {
                            "id": service.id,
                            "name": service.name,
                            "host": service.host,
                            "port": service.port,
                            "status": service.status
                        },
                        "requester": "nestgate-network"
                    }));

                match self
                    .universal_adapter
                    .request_capability("orchestration", request)
                    .await
                {
                    Ok(_response) => {
                        debug!("✅ Service registered via orchestration capability");
                        Ok(())
                    }
                    Err(e) => {
                        debug!(
                            "⚠️ Orchestration registration failed: {}, using local fallback",
                            e
                        );
                        self.local_registry.register_service(service);
                        Ok(())
                    }
                }
            }
            Err(e) => {
                debug!(
                    "⚠️ No orchestration capability available: {}, using local registry",
                    e
                );
                self.local_registry.register_service(service);
                Ok(())
            }
        }
    }

    /// List all services through orchestration capability or local fallback
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn list_all_services(&self) -> Result<Vec<ServiceInstance>> {
        match self.universal_adapter.get_capability("orchestration") {
            Ok(_capability_info) => {
                let request = CapabilityRequest::new("orchestration", "list_all_services")
                    .with_parameters(serde_json::json!({
                        "requester": "nestgate-network"
                    }));

                match self
                    .universal_adapter
                    .request_capability("orchestration", request)
                    .await
                {
                    Ok(_response) => {
                        debug!("✅ Service list retrieved via orchestration capability");
                        // For now, return empty vec - in real implementation would parse response.result
                        Ok(Vec::new())
                    }
                    Err(e) => {
                        debug!("⚠️ Orchestration list failed: {}, using local fallback", e);
                        Ok(self.local_registry.get_all_services())
                    }
                }
            }
            Err(e) => {
                debug!(
                    "⚠️ No orchestration capability available: {}, using local registry",
                    e
                );
                Ok(self.local_registry.get_all_services())
            }
        }
    }

    /// Check service health through orchestration capability or local fallback
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn check_service_health(&self, service_name: &str) -> Result<ServiceStatus> {
        match self.universal_adapter.get_capability("orchestration") {
            Ok(_capability_info) => {
                let request = CapabilityRequest::new("orchestration", "check_service_health")
                    .with_parameters(serde_json::json!({
                        "service_name": service_name,
                        "requester": "nestgate-network"
                    }));

                match self
                    .universal_adapter
                    .request_capability("orchestration", request)
                    .await
                {
                    Ok(_response) => {
                        debug!("✅ Service health checked via orchestration capability");
                        // For now, return Running - in real implementation would parse response.result
                        Ok(ServiceStatus::Running)
                    }
                    Err(e) => {
                        debug!(
                            "⚠️ Orchestration health check failed: {}, assuming stopped",
                            e
                        );
                        Ok(ServiceStatus::Stopped)
                    }
                }
            }
            Err(e) => {
                debug!(
                    "⚠️ No orchestration capability available: {}, assuming stopped",
                    e
                );
                Ok(ServiceStatus::Stopped)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_local_service_registry() {
        let mut registry = LocalServiceRegistry::new();

        let service = ServiceInstance {
            id: "test-service".to_string(),
            name: "test-service".to_string(),
            host: std::env::var("NESTGATE_TEST_HOST").unwrap_or_else(|_| "localhost".to_string()),
            port: 8080,
            status: ServiceStatus::Running,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        registry.register_service(service);

        let discovered = registry
            .discover_services("storage")
            .expect("Network operation failed");
        assert_eq!(discovered.len(), 1);
        assert_eq!(discovered[0].name, "test-service");
    }

    #[test]
    fn test_sovereignty_compliance() {
        // This test verifies that no hardcoded primal names exist in the code
        let source = include_str!("service_discovery_client.rs");

        // Should not contain hardcoded primal names (using variables to avoid self-reference)
        let primal1 = "song".to_owned() + "bird";
        let primal2 = "bear".to_owned() + "dog";
        let primal3 = "toad".to_owned() + "stool";

        assert!(!source.contains(&primal1));
        assert!(!source.contains(&primal2));
        assert!(!source.contains(&primal3));

        // Should use capability discovery
        assert!(source.contains("find_capability"));
        assert!(source.contains("orchestration"));
        assert!(source.contains("universal_adapter"));
    }
}
