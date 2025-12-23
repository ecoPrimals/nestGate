//! Capability registry for service discovery
//!
//! Central registry of all discovered services and their capabilities.
//! Supports concurrent access and dynamic service registration.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::service_descriptor::ServiceDescriptor;
use super::taxonomy::Capability;
use super::CapabilityResult;

/// Central registry of service capabilities
///
/// # Example
///
/// ```rust
/// use nestgate_core::capabilities::discovery::{CapabilityRegistry, ServiceDescriptor, Capability};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let registry = CapabilityRegistry::new();
///
/// // Register a service
/// // registry.register_service(service).await?;
///
/// // Find providers
/// // let providers = registry.find_providers(&Capability::Security).await;
/// # Ok(())
/// # }
/// ```
pub struct CapabilityRegistry {
    /// Map of capabilities to service providers
    capabilities: Arc<RwLock<HashMap<Capability, Vec<ServiceDescriptor>>>>,
}

impl CapabilityRegistry {
    /// Create a new capability registry
    pub fn new() -> Self {
        Self {
            capabilities: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a service with its capabilities
    pub async fn register_service(&self, service: ServiceDescriptor) -> CapabilityResult<()> {
        let mut capabilities = self.capabilities.write().await;

        for capability in &service.capabilities {
            capabilities
                .entry(capability.clone())
                .or_insert_with(Vec::new)
                .push(service.clone());
        }

        Ok(())
    }

    /// Find all services providing a capability
    pub async fn find_providers(&self, capability: &Capability) -> Vec<ServiceDescriptor> {
        let capabilities = self.capabilities.read().await;
        capabilities.get(capability).cloned().unwrap_or_default()
    }

    /// Check if any service provides a capability
    pub async fn has_capability(&self, capability: &Capability) -> bool {
        let capabilities = self.capabilities.read().await;
        capabilities.contains_key(capability)
    }

    /// Remove a service from the registry
    pub async fn unregister_service(&self, service_id: &uuid::Uuid) -> CapabilityResult<()> {
        let mut capabilities = self.capabilities.write().await;

        // Remove service from all capability lists
        for providers in capabilities.values_mut() {
            providers.retain(|s| &s.id != service_id);
        }

        // Remove empty capability entries
        capabilities.retain(|_, providers| !providers.is_empty());

        Ok(())
    }

    /// Get all registered services
    pub async fn all_services(&self) -> Vec<ServiceDescriptor> {
        let capabilities = self.capabilities.read().await;
        let mut services = Vec::new();
        let mut seen = std::collections::HashSet::new();

        for providers in capabilities.values() {
            for service in providers {
                if seen.insert(service.id) {
                    services.push(service.clone());
                }
            }
        }

        services
    }
}

impl Default for CapabilityRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::super::service_descriptor::{Endpoint, Protocol, ServiceHealth, ServiceMetadata};
    use super::super::taxonomy::{Capability, SecurityCapability};
    use super::*;

    #[tokio::test]
    async fn test_register_and_find_service() {
        let registry = CapabilityRegistry::new();

        let service = ServiceDescriptor {
            id: uuid::Uuid::new_v4(),
            name: "test-security".to_string(),
            capabilities: vec![Capability::Security(SecurityCapability::Authentication)],
            endpoint: Endpoint {
                host: "localhost".to_string(),
                port: crate::constants::network_hardcoded::ports::HTTPS_DEFAULT,
                protocol: Protocol::HTTPS,
                tls: true,
            },
            metadata: ServiceMetadata::default(),
            health: ServiceHealth::Healthy,
        };

        registry.register_service(service.clone()).await.unwrap();

        let providers = registry
            .find_providers(&Capability::Security(SecurityCapability::Authentication))
            .await;

        assert_eq!(providers.len(), 1);
        assert_eq!(providers[0].id, service.id);
    }

    #[tokio::test]
    async fn test_has_capability() {
        let registry = CapabilityRegistry::new();

        let service = ServiceDescriptor {
            id: uuid::Uuid::new_v4(),
            name: "test".to_string(),
            capabilities: vec![Capability::Security(SecurityCapability::Encryption)],
            endpoint: Endpoint::http("localhost".to_string(), 8080),
            metadata: ServiceMetadata::default(),
            health: ServiceHealth::Healthy,
        };

        registry.register_service(service).await.unwrap();

        assert!(
            registry
                .has_capability(&Capability::Security(SecurityCapability::Encryption))
                .await
        );

        assert!(
            !registry
                .has_capability(&Capability::Security(SecurityCapability::KeyManagement))
                .await
        );
    }

    #[tokio::test]
    async fn test_unregister_service() {
        let registry = CapabilityRegistry::new();

        let service_id = uuid::Uuid::new_v4();
        let service = ServiceDescriptor {
            id: service_id,
            name: "test".to_string(),
            capabilities: vec![Capability::Security(SecurityCapability::Authentication)],
            endpoint: Endpoint::http("localhost".to_string(), 8080),
            metadata: ServiceMetadata::default(),
            health: ServiceHealth::Healthy,
        };

        registry.register_service(service).await.unwrap();
        assert!(
            registry
                .has_capability(&Capability::Security(SecurityCapability::Authentication))
                .await
        );

        registry.unregister_service(&service_id).await.unwrap();
        assert!(
            !registry
                .has_capability(&Capability::Security(SecurityCapability::Authentication))
                .await
        );
    }

    #[tokio::test]
    async fn test_multiple_services_same_capability() {
        use super::super::taxonomy::StorageCapability;

        let registry = CapabilityRegistry::new();

        // Register multiple services with the same capability
        let service1 = ServiceDescriptor {
            id: uuid::Uuid::new_v4(),
            name: "storage1".to_string(),
            capabilities: vec![Capability::Storage(StorageCapability::ObjectStorage)],
            endpoint: Endpoint::http("localhost".to_string(), 9001),
            metadata: ServiceMetadata::default(),
            health: ServiceHealth::Healthy,
        };

        let service2 = ServiceDescriptor {
            id: uuid::Uuid::new_v4(),
            name: "storage2".to_string(),
            capabilities: vec![Capability::Storage(StorageCapability::ObjectStorage)],
            endpoint: Endpoint::http("localhost".to_string(), 9002),
            metadata: ServiceMetadata::default(),
            health: ServiceHealth::Healthy,
        };

        registry.register_service(service1.clone()).await.unwrap();
        registry.register_service(service2.clone()).await.unwrap();

        let providers = registry
            .find_providers(&Capability::Storage(StorageCapability::ObjectStorage))
            .await;

        assert_eq!(providers.len(), 2);
        assert!(providers.iter().any(|s| s.id == service1.id));
        assert!(providers.iter().any(|s| s.id == service2.id));
    }

    #[tokio::test]
    async fn test_service_with_multiple_capabilities() {
        use super::super::taxonomy::{NetworkingCapability, OrchestrationCapability};

        let registry = CapabilityRegistry::new();

        let service = ServiceDescriptor {
            id: uuid::Uuid::new_v4(),
            name: "multi-cap".to_string(),
            capabilities: vec![
                Capability::Networking(NetworkingCapability::LoadBalancing),
                Capability::Orchestration(OrchestrationCapability::ServiceScheduling),
            ],
            endpoint: Endpoint::http("localhost".to_string(), 8080),
            metadata: ServiceMetadata::default(),
            health: ServiceHealth::Healthy,
        };

        registry.register_service(service.clone()).await.unwrap();

        // Should find service under both capabilities
        let providers1 = registry
            .find_providers(&Capability::Networking(NetworkingCapability::LoadBalancing))
            .await;
        assert_eq!(providers1.len(), 1);
        assert_eq!(providers1[0].id, service.id);

        let providers2 = registry
            .find_providers(&Capability::Orchestration(
                OrchestrationCapability::ServiceScheduling,
            ))
            .await;
        assert_eq!(providers2.len(), 1);
        assert_eq!(providers2[0].id, service.id);
    }

    #[tokio::test]
    async fn test_empty_registry() {
        let registry = CapabilityRegistry::new();

        let providers = registry
            .find_providers(&Capability::Security(SecurityCapability::Authentication))
            .await;

        assert_eq!(providers.len(), 0);
        assert!(
            !registry
                .has_capability(&Capability::Security(SecurityCapability::Authentication))
                .await
        );
    }

    #[tokio::test]
    async fn test_unregister_nonexistent_service() {
        let registry = CapabilityRegistry::new();

        // Should not error when unregistering non-existent service
        let result = registry.unregister_service(&uuid::Uuid::new_v4()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_service() {
        use super::super::taxonomy::AICapability;

        let registry = CapabilityRegistry::new();

        let service_id = uuid::Uuid::new_v4();

        // Register initial service
        let service1 = ServiceDescriptor {
            id: service_id,
            name: "ai-service".to_string(),
            capabilities: vec![Capability::AI(AICapability::Inference)],
            endpoint: Endpoint::http("localhost".to_string(), 8080),
            metadata: ServiceMetadata::default(),
            health: ServiceHealth::Healthy,
        };

        registry.register_service(service1).await.unwrap();

        // Update service (unregister and re-register)
        registry.unregister_service(&service_id).await.unwrap();

        let service2 = ServiceDescriptor {
            id: service_id,
            name: "ai-service-v2".to_string(),
            capabilities: vec![Capability::AI(AICapability::Training)],
            endpoint: Endpoint::http("localhost".to_string(), 8081),
            metadata: ServiceMetadata::default(),
            health: ServiceHealth::Healthy,
        };

        registry.register_service(service2).await.unwrap();

        // Should have new capability, not old one
        assert!(
            !registry
                .has_capability(&Capability::AI(AICapability::Inference))
                .await
        );
        assert!(
            registry
                .has_capability(&Capability::AI(AICapability::Training))
                .await
        );
    }

    #[tokio::test]
    async fn test_list_all_capabilities() {
        use super::super::taxonomy::{AICapability, StorageCapability};

        let registry = CapabilityRegistry::new();

        let service1 = ServiceDescriptor {
            id: uuid::Uuid::new_v4(),
            name: "storage".to_string(),
            capabilities: vec![Capability::Storage(StorageCapability::Database)],
            endpoint: Endpoint::http("localhost".to_string(), 5432),
            metadata: ServiceMetadata::default(),
            health: ServiceHealth::Healthy,
        };

        let service2 = ServiceDescriptor {
            id: uuid::Uuid::new_v4(),
            name: "ai".to_string(),
            capabilities: vec![Capability::AI(AICapability::ModelServing)],
            endpoint: Endpoint::http("localhost".to_string(), 8080),
            metadata: ServiceMetadata::default(),
            health: ServiceHealth::Healthy,
        };

        registry.register_service(service1).await.unwrap();
        registry.register_service(service2).await.unwrap();

        // Verify both services registered
        assert!(
            registry
                .has_capability(&Capability::Storage(StorageCapability::Database))
                .await
        );
        assert!(
            registry
                .has_capability(&Capability::AI(AICapability::ModelServing))
                .await
        );
    }

    #[tokio::test]
    async fn test_registry_persistence() {
        let registry = CapabilityRegistry::new();

        let service = ServiceDescriptor {
            id: uuid::Uuid::new_v4(),
            name: "test".to_string(),
            capabilities: vec![Capability::Security(SecurityCapability::Authentication)],
            endpoint: Endpoint::http("localhost".to_string(), 8080),
            metadata: ServiceMetadata::default(),
            health: ServiceHealth::Healthy,
        };

        registry.register_service(service.clone()).await.unwrap();

        // Query multiple times - should persist
        for _ in 0..3 {
            assert!(
                registry
                    .has_capability(&Capability::Security(SecurityCapability::Authentication))
                    .await
            );
            let providers = registry
                .find_providers(&Capability::Security(SecurityCapability::Authentication))
                .await;
            assert_eq!(providers.len(), 1);
            assert_eq!(providers[0].id, service.id);
        }
    }

    #[tokio::test]
    async fn test_concurrent_registration() {
        use super::super::taxonomy::NetworkingCapability;

        let registry = Arc::new(CapabilityRegistry::new());

        // Create multiple services
        let services: Vec<_> = (0..5)
            .map(|i| ServiceDescriptor {
                id: uuid::Uuid::new_v4(),
                name: format!("service{}", i),
                capabilities: vec![Capability::Networking(NetworkingCapability::HTTP)],
                endpoint: Endpoint::http("localhost".to_string(), 8080 + i),
                metadata: ServiceMetadata::default(),
                health: ServiceHealth::Healthy,
            })
            .collect();

        // Register concurrently
        let mut handles = vec![];
        for service in services {
            let registry = Arc::clone(&registry);
            let handle = tokio::spawn(async move { registry.register_service(service).await });
            handles.push(handle);
        }

        // Wait for all
        for handle in handles {
            handle.await.unwrap().unwrap();
        }

        // Should have all 5 services
        let providers = registry
            .find_providers(&Capability::Networking(NetworkingCapability::HTTP))
            .await;
        assert_eq!(providers.len(), 5);
    }

    #[tokio::test]
    async fn test_concurrent_read_write() {
        use super::super::taxonomy::OrchestrationCapability;

        let registry = Arc::new(CapabilityRegistry::new());

        let service = ServiceDescriptor {
            id: uuid::Uuid::new_v4(),
            name: "test".to_string(),
            capabilities: vec![Capability::Orchestration(
                OrchestrationCapability::HealthMonitoring,
            )],
            endpoint: Endpoint::http("localhost".to_string(), 8080),
            metadata: ServiceMetadata::default(),
            health: ServiceHealth::Healthy,
        };

        registry.register_service(service).await.unwrap();

        // Concurrent reads and writes
        let reg1 = Arc::clone(&registry);
        let reg2 = Arc::clone(&registry);
        let reg3 = Arc::clone(&registry);

        let (r1, r2, r3) = tokio::join!(
            async move {
                reg1.find_providers(&Capability::Orchestration(
                    OrchestrationCapability::HealthMonitoring,
                ))
                .await
            },
            async move {
                reg2.find_providers(&Capability::Orchestration(
                    OrchestrationCapability::HealthMonitoring,
                ))
                .await
            },
            async move {
                reg3.has_capability(&Capability::Orchestration(
                    OrchestrationCapability::HealthMonitoring,
                ))
                .await
            }
        );

        assert_eq!(r1.len(), 1);
        assert_eq!(r2.len(), 1);
        assert!(r3);
    }

    #[tokio::test]
    async fn test_find_all_services() {
        use super::super::taxonomy::{SecurityCapability, StorageCapability};

        let registry = CapabilityRegistry::new();

        let service1 = ServiceDescriptor {
            id: uuid::Uuid::new_v4(),
            name: "security".to_string(),
            capabilities: vec![Capability::Security(SecurityCapability::Encryption)],
            endpoint: Endpoint::http("localhost".to_string(), 8001),
            metadata: ServiceMetadata::default(),
            health: ServiceHealth::Healthy,
        };

        let service2 = ServiceDescriptor {
            id: uuid::Uuid::new_v4(),
            name: "storage".to_string(),
            capabilities: vec![Capability::Storage(StorageCapability::Cache)],
            endpoint: Endpoint::http("localhost".to_string(), 8002),
            metadata: ServiceMetadata::default(),
            health: ServiceHealth::Healthy,
        };

        registry.register_service(service1.clone()).await.unwrap();
        registry.register_service(service2.clone()).await.unwrap();

        let all_services = registry.all_services().await;
        assert_eq!(all_services.len(), 2);
        assert!(all_services.iter().any(|s| s.id == service1.id));
        assert!(all_services.iter().any(|s| s.id == service2.id));
    }

    #[tokio::test]
    async fn test_unregister_removes_from_all_capabilities() {
        use super::super::taxonomy::{AICapability, NetworkingCapability};

        let registry = CapabilityRegistry::new();

        let service_id = uuid::Uuid::new_v4();
        let service = ServiceDescriptor {
            id: service_id,
            name: "multi".to_string(),
            capabilities: vec![
                Capability::AI(AICapability::ComputerVision),
                Capability::Networking(NetworkingCapability::WebSocket),
            ],
            endpoint: Endpoint::http("localhost".to_string(), 8080),
            metadata: ServiceMetadata::default(),
            health: ServiceHealth::Healthy,
        };

        registry.register_service(service).await.unwrap();

        // Verify registered
        assert!(
            registry
                .has_capability(&Capability::AI(AICapability::ComputerVision))
                .await
        );
        assert!(
            registry
                .has_capability(&Capability::Networking(NetworkingCapability::WebSocket))
                .await
        );

        // Unregister
        registry.unregister_service(&service_id).await.unwrap();

        // Should be removed from all capabilities
        assert!(
            !registry
                .has_capability(&Capability::AI(AICapability::ComputerVision))
                .await
        );
        assert!(
            !registry
                .has_capability(&Capability::Networking(NetworkingCapability::WebSocket))
                .await
        );
    }

    #[tokio::test]
    async fn test_default_implementation() {
        let registry = CapabilityRegistry::default();

        // Should be empty
        let all_services = registry.all_services().await;
        assert_eq!(all_services.len(), 0);
    }

    #[tokio::test]
    async fn test_registry_clone_independence() {
        let registry1 = Arc::new(CapabilityRegistry::new());
        let registry2 = Arc::clone(&registry1);

        let service = ServiceDescriptor {
            id: uuid::Uuid::new_v4(),
            name: "test".to_string(),
            capabilities: vec![Capability::Security(SecurityCapability::AuditLogging)],
            endpoint: Endpoint::http("localhost".to_string(), 8080),
            metadata: ServiceMetadata::default(),
            health: ServiceHealth::Healthy,
        };

        // Register in one
        registry1.register_service(service.clone()).await.unwrap();

        // Should be visible in both (shared state)
        assert!(
            registry1
                .has_capability(&Capability::Security(SecurityCapability::AuditLogging))
                .await
        );
        assert!(
            registry2
                .has_capability(&Capability::Security(SecurityCapability::AuditLogging))
                .await
        );
    }

    #[test]
    fn test_registry_creation() {
        let _registry = CapabilityRegistry::new();
        // Should not panic
    }

    #[tokio::test]
    async fn test_empty_capability_query() {
        let registry = CapabilityRegistry::new();

        // Query non-existent custom capability
        let providers = registry
            .find_providers(&Capability::Custom("NonExistent".to_string()))
            .await;
        assert_eq!(providers.len(), 0);
    }
}
