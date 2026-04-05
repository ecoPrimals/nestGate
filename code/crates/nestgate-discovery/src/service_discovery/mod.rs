// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

// Removed unused import: std::collections::HashMap
// Universal Service Discovery Module - Split for File Size Compliance
// This module was split from universal_service_discovery.rs to maintain the 2000-line limit
//! while preserving all functionality and maintaining backward compatibility.
// **ARCHITECTURAL PRINCIPLE**: "Systems should discover and integrate based on what they can do, not what they're called"

// Sub-module declarations
pub mod dynamic_endpoints;
pub mod dynamic_endpoints_config; // ✅ NEW: Concurrent-safe configuration
/// Service registry implementation
pub mod registry;
/// Service discovery types
pub mod types;

#[cfg(test)]
mod types_tests;

#[cfg(test)]
mod discovery_error_tests; // Nov 23, 2025 - P1 test expansion
#[cfg(test)]
mod service_discovery_edge_cases; // Nov 23, 2025 - P1-5 edge case tests // ✅ NEW: Dynamic endpoint resolution system

// Re-export all public types for backward compatibility
pub use dynamic_endpoints::{DynamicEndpointResolver, resolve_service_endpoint};
pub use dynamic_endpoints_config::{DynamicEndpointsConfig, SharedEndpointsConfig}; // ✅ NEW: Export config
pub use types::*; // ✅ NEW: Export dynamic endpoint functionality

// Convenience re-exports for common usage patterns
pub use crate::service_discovery::registry::{InMemoryServiceRegistry, UniversalServiceRegistry};

// Backward compatibility aliases for legacy code
/// Type alias for the universal service registry trait object.
///
/// This provides a dynamic interface for service discovery implementations,
/// allowing runtime polymorphism for different registry backends.
pub type ServiceDiscovery = dyn UniversalServiceRegistry;
/// Type alias for Serviceregistry
pub type ServiceRegistry = InMemoryServiceRegistry;

// Convenience function to create a new service registry
#[must_use]
/// Creates a new in-memory service registry.
///
/// Returns an in-memory implementation of the service registry suitable for
/// development, testing, and single-node deployments.
pub fn create_service_registry() -> InMemoryServiceRegistry {
    InMemoryServiceRegistry::new()
}
// Convenience function to create a universal service registration
#[must_use]
/// Creates a service registration with the specified name and capabilities.
///
/// # Arguments
/// * `name` - The unique name for this service
/// * `_category` - The service category (reserved for future use)
/// * `capabilities` - List of capabilities provided by this service
pub fn create_service_registration(
    name: String,
    _category: ServiceCategory,
    capabilities: Vec<ServiceCapability>,
) -> UniversalServiceRegistration {
    let metadata = crate::service_discovery::types::ServiceMetadata {
        name,
        ..Default::default()
    };

    UniversalServiceRegistration {
        service_id: uuid::Uuid::new_v4(),
        metadata,
        capabilities,          // Use the provided capabilities
        endpoints: Vec::new(), // Endpoints will be discovered dynamically
        resources: ResourceSpec::default(),
        integration: IntegrationPreferences::default(),
        extensions: std::collections::HashMap::new(),
    }
}
// Create a service role for common patterns
#[must_use]
/// Creates a storage service role with standard permissions
pub fn create_storage_role() -> ServiceRole {
    ServiceRole {
        name: "Storage Provider".to_string(),
        required_capabilities: vec![ServiceCapability::Storage(StorageType::Object)],
        optional_capabilities: vec![ServiceCapability::Custom {
            namespace: "cache-provider".to_string(),
            capability: "Cache Provider".to_string(),
            version: "1.0.0".to_string(),
        }],
        resource_requirements: ResourceSpec::default(),
        performance_requirements: PerformanceRequirements::default(),
    }
}
// Create a service role for AI services
#[must_use]
/// Creates an AI service role with standard permissions
pub fn create_ai_role() -> ServiceRole {
    ServiceRole {
        name: "AI Provider".to_string(),
        required_capabilities: vec![ServiceCapability::Custom {
            namespace: "ai-provider".to_string(),
            capability: "AI Provider".to_string(),
            version: "1.0.0".to_string(),
        }],
        optional_capabilities: vec![
            ServiceCapability::Custom {
                namespace: "nlp-provider".to_string(),
                capability: "NLP Provider".to_string(),
                version: "1.0.0".to_string(),
            },
            ServiceCapability::Custom {
                namespace: "data-processing".to_string(),
                capability: "Data Processing".to_string(),
                version: "1.0.0".to_string(),
            },
        ],
        resource_requirements: ResourceSpec {
            cpu_cores: Some(2.0),
            memory_mb: Some(4096),
            disk_gb: Some(20),
            network_mbps: Some(100),
            constraints: ResourceConstraints::default(),
        },
        performance_requirements: PerformanceRequirements {
            max_latency_ms: Some(5000),
            min_throughput_rps: Some(10),
            availability_percent: Some(99.5),
        },
    }
}
// Create a service role for security services
#[must_use]
/// Creates a security service role with standard permissions
pub fn create_security_role() -> ServiceRole {
    ServiceRole {
        name: "Security Provider".to_string(),
        required_capabilities: vec![ServiceCapability::Custom {
            namespace: "security-provider".to_string(),
            capability: "Security Provider".to_string(),
            version: "1.0.0".to_string(),
        }],
        optional_capabilities: vec![
            ServiceCapability::Custom {
                namespace: "authorization".to_string(),
                capability: "Authorization".to_string(),
                version: "1.0.0".to_string(),
            },
            ServiceCapability::Custom {
                namespace: "encryption".to_string(),
                capability: "Encryption".to_string(),
                version: "1.0.0".to_string(),
            },
        ],
        resource_requirements: ResourceSpec::default(),
        performance_requirements: PerformanceRequirements {
            max_latency_ms: Some(1000),
            min_throughput_rps: Some(100),
            availability_percent: Some(99.9),
        },
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_service_registry_creation() {
        let registry = create_service_registry();
        assert_eq!(registry.service_count(), 0);
    }

    #[tokio::test]
    async fn test_service_registration() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let registry = create_service_registry();
        let registration = create_service_registration(
            // SOVEREIGNTY FIX: Use capability-based naming
            "test-capability".to_string(),
            ServiceCategory::Storage,
            vec![ServiceCapability::Storage(StorageType::Object)],
        );

        let handle = registry.register_service(registration).await?;
        // SOVEREIGNTY FIX: Updated to match capability-based naming
        assert_eq!(handle.name, "test-capability");
        assert_eq!(registry.service_count(), 1);
        Ok(())
    }

    #[tokio::test]
    async fn test_capability_discovery() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let registry = create_service_registry();
        let registration = create_service_registration(
            // SOVEREIGNTY FIX: Use capability-based naming
            "storage-capability".to_string(),
            ServiceCategory::Storage,
            vec![ServiceCapability::Storage(StorageType::Object)],
        );

        registry.register_service(registration).await?;

        let services = registry
            .discover_by_capabilities(vec![ServiceCapability::Storage(StorageType::Object)])
            .await?;

        assert_eq!(services.len(), 1);
        // SOVEREIGNTY FIX: Updated to match capability-based naming
        assert_eq!(services[0].metadata.name, "storage-capability");
        Ok(())
    }

    #[tokio::test]
    async fn test_role_based_discovery() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let registry = create_service_registry();
        let storage_role = create_storage_role();

        let registration = create_service_registration(
            "object-storage".to_string(),
            ServiceCategory::Storage,
            vec![ServiceCapability::Storage(StorageType::Object)],
        );

        registry.register_service(registration).await?;

        let services = registry.discover_by_role(storage_role).await?;
        assert_eq!(services.len(), 1);
        assert_eq!(services[0].metadata.name, "object-storage");
        Ok(())
    }

    #[tokio::test]
    async fn test_optimal_service_selection() -> std::result::Result<(), Box<dyn std::error::Error>>
    {
        let registry = create_service_registry();

        // Register multiple services
        for i in 0..3 {
            let registration = create_service_registration(
                format!("service-{i}"),
                ServiceCategory::Storage,
                vec![crate::service_discovery::types::ServiceCapability::Storage(
                    crate::service_discovery::types::StorageType::Object,
                )],
            );
            registry.register_service(registration).await?;
        }

        let requirements = ServiceRequirements {
            capabilities: vec![ServiceCapability::Storage(StorageType::Object)],
            resource_constraints: None,
            performance_requirements: None,
        };

        let preferences = SelectionPreferences::default();
        let optimal_service = registry
            .find_optimal_service(requirements, preferences)
            .await?;

        // Should return one of the registered services
        assert!(optimal_service.metadata.name.starts_with("service-"));
        Ok(())
    }
}
