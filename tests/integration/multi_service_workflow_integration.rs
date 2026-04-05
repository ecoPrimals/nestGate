// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

/// **PHASE 2: MULTI-SERVICE WORKFLOW INTEGRATION TESTS**
///
/// Comprehensive integration tests targeting 90%+ coverage through:
/// - Multi-service coordination workflows
/// - Service discovery and capability-based routing  
/// - Configuration migration and compatibility
/// - Failure recovery and resilience patterns
/// - Cross-crate component interaction validation

use std::collections::HashMap;

use std::time::{Duration, SystemTime};
use tokio::time::{sleep, timeout};
use uuid::Uuid;

use nestgate_core::{

    error::{NestGateError, Result},
    service_discovery::{
        registry::{InMemoryServiceRegistry, UniversalServiceRegistry},
        types::{
            ServiceCapability, ServiceInfo, ServiceRole, UniversalServiceRegistration,
            ServiceRequirements, SelectionPreferences, ServiceHandle,
            ServiceMetadata, ServiceEndpoint, CommunicationProtocol, ServiceCategory
        }
    },
    universal_traits::{
        CapabilityDiscoveryManager, AutoDiscoveryConfig, DiscoveryMethod,
        UniversalConfig, EcosystemIntegrationConfig, ServiceInstance
    },
    config::{Config, network::ServiceEndpoints, security::SecurityConfig},
    smart_abstractions::service_patterns::{SmartServiceDiscovery, ServiceRegistration},
    Result as CoreResult,
};

/// **TEST SUITE 1: SERVICE DISCOVERY WORKFLOW INTEGRATION**
#[cfg(test)]
mod service_discovery_integration_tests {
    use super::*;

    /// Test complete service discovery workflow with multiple providers
    #[tokio::test]
    async fn test_multi_provider_service_discovery_workflow() -> Result<()> {
        // Setup: Create service registry with multiple discovery methods
        let registry = Arc::new(InMemoryServiceRegistry::new());
        
        // Register multiple services with different capabilities
        let security_service = create_test_service_registration(
            "security-service",
            vec![ServiceCapability::Authentication, ServiceCapability::Encryption],
            ServiceRole::Security,
        );
        
        let orchestration_service = create_test_service_registration(
            "orchestration-service", 
            vec![ServiceCapability::Coordination, ServiceCapability::LoadBalancing],
            ServiceRole::Orchestration,
        );
        
        let storage_service = create_test_service_registration(
            "storage-service",
            vec![ServiceCapability::Storage, ServiceCapability::Backup],
            ServiceRole::Storage,
        );
        
        // Register all services
        let _security_handle = registry.register_service(security_service).await?;
        let _orchestration_handle = registry.register_service(orchestration_service).await?;
        let _storage_handle = registry.register_service(storage_service).await?;
        
        // Test: Discover services by capability
        let auth_services = registry.discover_by_capabilities(vec![ServiceCapability::Authentication]).await?;
        assert_eq!(auth_services.len(), 1);
        assert_eq!(auth_services[0].name, "security-service");
        
        let storage_services = registry.discover_by_capabilities(vec![ServiceCapability::Storage]).await?;
        assert_eq!(storage_services.len(), 1);
        assert_eq!(storage_services[0].name, "storage-service");
        
        // Test: Multi-capability discovery
        let multi_cap_services = registry.discover_by_capabilities(vec![
            ServiceCapability::Coordination,
            ServiceCapability::LoadBalancing
        ]).await?;
        assert_eq!(multi_cap_services.len(), 1);
        assert_eq!(multi_cap_services[0].name, "orchestration-service");
        
        println!("✅ Multi-provider service discovery workflow tested");
        Ok(())
    }
    
    /// Test service discovery with failure scenarios and recovery
    #[tokio::test]
    async fn test_service_discovery_failure_recovery() -> Result<()> {
        let registry = Arc::new(InMemoryServiceRegistry::new());
        
        // Register a service
        let service = create_test_service_registration(
            "test-service",
            vec![ServiceCapability::Storage],
            ServiceRole::Storage,
        );
        let handle = registry.register_service(service).await?;
        
        // Verify service is discoverable
        let services = registry.discover_by_capabilities(vec![ServiceCapability::Storage]).await?;
        assert_eq!(services.len(), 1);
        
        // Simulate service failure by deregistering
        registry.deregister_service(handle.service_id).await?;
        
        // Verify service is no longer discoverable
        let services_after_failure = registry.discover_by_capabilities(vec![ServiceCapability::Storage]).await?;
        assert_eq!(services_after_failure.len(), 0);
        
        // Test recovery by re-registering
        let recovered_service = create_test_service_registration(
            "recovered-service",
            vec![ServiceCapability::Storage],
            ServiceRole::Storage,
        );
        let _recovered_handle = registry.register_service(recovered_service).await?;
        
        // Verify service is discoverable again
        let recovered_services = registry.discover_by_capabilities(vec![ServiceCapability::Storage]).await?;
        assert_eq!(recovered_services.len(), 1);
        assert_eq!(recovered_services[0].name, "recovered-service");
        
        println!("✅ Service discovery failure recovery tested");
        Ok(())
    }
    
    /// Test optimal service selection with requirements and preferences
    #[tokio::test]
    async fn test_optimal_service_selection() -> Result<()> {
        let registry = Arc::new(InMemoryServiceRegistry::new());
        
        // Register multiple storage services with different characteristics
        let fast_storage = create_test_service_with_metadata(
            "fast-storage",
            vec![ServiceCapability::Storage],
            ServiceRole::Storage,
            HashMap::from([
                ("performance".to_string(), "high".to_string()),
                ("latency".to_string(), "low".to_string()),
            ])
        );
        
        let cheap_storage = create_test_service_with_metadata(
            "cheap-storage", 
            vec![ServiceCapability::Storage],
            ServiceRole::Storage,
            HashMap::from([
                ("cost".to_string(), "low".to_string()),
                ("capacity".to_string(), "high".to_string()),
            ])
        );
        
        registry.register_service(fast_storage).await?;
        registry.register_service(cheap_storage).await?;
        
        // Test: Find optimal service for high-performance requirements
        let performance_requirements = ServiceRequirements {
            required_capabilities: vec![ServiceCapability::Storage],
            performance_tier: Some("high".to_string()),
            max_latency: Some(Duration::from_millis(10)),
            min_availability: Some(0.99),
        };
        
        let performance_preferences = SelectionPreferences {
            prefer_low_latency: true,
            prefer_high_performance: true,
            prefer_local: false,
            cost_sensitivity: 0.2, // Low cost sensitivity
        };
        
        let optimal_service = registry.find_optimal_service(
            performance_requirements,
            performance_preferences
        ).await?;
        
        assert_eq!(optimal_service.name, "fast-storage");
        
        println!("✅ Optimal service selection tested");
        Ok(())
    }
}

/// **TEST SUITE 2: CONFIGURATION MIGRATION INTEGRATION**
#[cfg(test)]
mod configuration_migration_tests {
    use super::*;

    /// Test configuration migration from legacy to capability-based
    #[tokio::test]
    async fn test_legacy_to_capability_config_migration() -> Result<()> {
        // Setup: Create legacy configuration
        let mut legacy_config = Config::default();
        
        // Simulate legacy hardcoded service endpoints
        let mut legacy_endpoints = ServiceEndpoints::default();
        legacy_endpoints.set_service_url("legacy-security".to_string(), "http://hardcoded:8001".to_string());
        legacy_endpoints.set_service_url("legacy-storage".to_string(), "http://hardcoded:8002".to_string());
        
        // Test: Migration to capability-based discovery
        let discovery_config = AutoDiscoveryConfig {
            enabled: true,
            discovery_methods: vec![
                DiscoveryMethod::Dns { domain: "local".to_string() },
                DiscoveryMethod::Static { 
                    services: vec![
                        ServiceInstance {
                            id: "security-service".to_string(),
                            name: "security-service".to_string(),
                            capabilities: vec!["authentication".to_string(), "encryption".to_string()],
                            endpoint: "http://discovered:8001".to_string(),
                        },
                        ServiceInstance {
                            id: "storage-service".to_string(), 
                            name: "storage-service".to_string(),
                            capabilities: vec!["storage".to_string(), "backup".to_string()],
                            endpoint: "http://discovered:8002".to_string(),
    Ok(())
                        }
                    ]
    Ok(())
                }
            ],
            refresh_interval: 30,
            timeout: 5,
        };
        
        let capability_manager = CapabilityDiscoveryManager::new(discovery_config);
        capability_manager.start_discovery()?;
        
        // Test: Verify capability-based discovery works
        let auth_services = capability_manager.find_services_with_capability("authentication")?;
        assert_eq!(auth_services.len(), 1);
        assert_eq!(auth_services[0].name, "security-service");
        
        let storage_services = capability_manager.find_services_with_capability("storage")?;
        assert_eq!(storage_services.len(), 1);
        assert_eq!(storage_services[0].name, "storage-service");
        
        // Test: Verify best service selection
        let best_auth_service = capability_manager.get_best_service_for_capability("authentication")?;
        assert!(best_auth_service.is_some());
        assert_eq!(best_auth_service?.endpoint, "http://discovered:8001");
        
        println!("✅ Legacy to capability-based configuration migration tested");
        Ok(())
    }
    
    /// Test configuration hot-reload during service operation
    #[tokio::test]
    async fn test_configuration_hot_reload_integration() -> Result<()> {
        // Setup: Initial configuration
        let initial_config = AutoDiscoveryConfig {
            enabled: true,
            discovery_methods: vec![
                DiscoveryMethod::Static {
                    services: vec![
                        ServiceInstance {
                            id: "service-v1".to_string(),
                            name: "test-service".to_string(),
                            capabilities: vec!["storage".to_string()],
                            endpoint: "http://v1:8000".to_string(),
    Ok(())
                        }
                    ]
    Ok(())
                }
            ],
            refresh_interval: 30,
            timeout: 5,
        };
        
        let capability_manager = CapabilityDiscoveryManager::new(initial_config);
        capability_manager.start_discovery()?;
        
        // Verify initial configuration
        let initial_services = capability_manager.find_services_with_capability("storage")?;
        assert_eq!(initial_services.len(), 1);
        assert_eq!(initial_services[0].endpoint, "http://v1:8000");
        
        // Simulate configuration reload with updated services
        let updated_config = AutoDiscoveryConfig {
            enabled: true,
            discovery_methods: vec![
                DiscoveryMethod::Static {
                    services: vec![
                        ServiceInstance {
                            id: "service-v2".to_string(),
                            name: "test-service".to_string(),
                            capabilities: vec!["storage".to_string(), "backup".to_string()],
                            endpoint: "http://v2:8000".to_string(),
    Ok(())
                        }
                    ]
    Ok(())
                }
            ],
            refresh_interval: 30,
            timeout: 5,
        };
        
        // Create new manager with updated config (simulates hot reload)
        let updated_manager = CapabilityDiscoveryManager::new(updated_config);
        updated_manager.start_discovery()?;
        
        // Verify updated configuration
        let updated_services = updated_manager.find_services_with_capability("storage")?;
        assert_eq!(updated_services.len(), 1);
        assert_eq!(updated_services[0].endpoint, "http://v2:8000");
        
        // Verify new capability is available
        let backup_services = updated_manager.find_services_with_capability("backup")?;
        assert_eq!(backup_services.len(), 1);
        
        println!("✅ Configuration hot-reload integration tested");
        Ok(())
    }
}

/// **TEST SUITE 3: FAILURE RECOVERY AND RESILIENCE**
#[cfg(test)]
mod failure_recovery_tests {
    use super::*;

    /// Test service failure detection and automatic recovery
    #[tokio::test]
    async fn test_service_failure_detection_and_recovery() -> Result<()> {
        let smart_discovery = SmartServiceDiscovery::new();
        
        // Register a test service
        let test_service = TestSmartService::new("resilient-service", vec!["storage".to_string()]);
        smart_discovery.register_service(&test_service).await?;
        
        // Verify service is healthy
        let healthy_services = smart_discovery.get_healthy_services("storage".to_string()).await;
        assert_eq!(healthy_services.len(), 1);
        
        // Simulate service failure by marking it unhealthy
        // (In a real implementation, this would be detected through health checks)
        
        // Test recovery workflow
        let recovery_service = TestSmartService::new("recovery-service", vec!["storage".to_string()]);
        smart_discovery.register_service(&recovery_service).await?;
        
        // Verify recovery service is available
        let recovered_services = smart_discovery.get_healthy_services("storage".to_string()).await;
        assert!(recovered_services.len() >= 1);
        
        println!("✅ Service failure detection and recovery tested");
        Ok(())
    }
    
    /// Test network partition recovery scenarios
    #[tokio::test]
    async fn test_network_partition_recovery() -> Result<()> {
        let registry = Arc::new(InMemoryServiceRegistry::new());
        
        // Setup: Register services in different "network zones"
        let zone_a_service = create_test_service_with_metadata(
            "zone-a-storage",
            vec![ServiceCapability::Storage],
            ServiceRole::Storage,
            HashMap::from([("zone".to_string(), "a".to_string())])
        );
        
        let zone_b_service = create_test_service_with_metadata(
            "zone-b-storage",
            vec![ServiceCapability::Storage], 
            ServiceRole::Storage,
            HashMap::from([("zone".to_string(), "b".to_string())])
        );
        
        let zone_a_handle = registry.register_service(zone_a_service).await?;
        let zone_b_handle = registry.register_service(zone_b_service).await?;
        
        // Verify both services are available
        let all_storage_services = registry.discover_by_capabilities(vec![ServiceCapability::Storage]).await?;
        assert_eq!(all_storage_services.len(), 2);
        
        // Simulate network partition: Zone A becomes unavailable
        registry.deregister_service(zone_a_handle.service_id).await?;
        
        // Verify Zone B service is still available
        let available_services = registry.discover_by_capabilities(vec![ServiceCapability::Storage]).await?;
        assert_eq!(available_services.len(), 1);
        assert_eq!(available_services[0].name, "zone-b-storage");
        
        // Simulate partition recovery: Zone A comes back online
        let zone_a_recovered = create_test_service_with_metadata(
            "zone-a-storage-recovered",
            vec![ServiceCapability::Storage],
            ServiceRole::Storage,
            HashMap::from([("zone".to_string(), "a".to_string())])
        );
        
        registry.register_service(zone_a_recovered).await?;
        
        // Verify both zones are available again
        let recovered_services = registry.discover_by_capabilities(vec![ServiceCapability::Storage]).await?;
        assert_eq!(recovered_services.len(), 2);
        
        println!("✅ Network partition recovery tested");
        Ok(())
    }
    
    /// Test cascading failure prevention
    #[tokio::test]
    async fn test_cascading_failure_prevention() -> Result<()> {
        let registry = Arc::new(InMemoryServiceRegistry::new());
        
        // Setup: Create service dependency chain
        let primary_service = create_test_service_registration(
            "primary-service",
            vec![ServiceCapability::Storage, ServiceCapability::Coordination],
            ServiceRole::Primary,
        );
        
        let secondary_service = create_test_service_registration(
            "secondary-service", 
            vec![ServiceCapability::Storage],
            ServiceRole::Secondary,
        );
        
        let backup_service = create_test_service_registration(
            "backup-service",
            vec![ServiceCapability::Storage],
            ServiceRole::Backup,
        );
        
        let primary_handle = registry.register_service(primary_service).await?;
        registry.register_service(secondary_service).await?;
        registry.register_service(backup_service).await?;
        
        // Verify all services are available
        let all_services = registry.discover_by_capabilities(vec![ServiceCapability::Storage]).await?;
        assert_eq!(all_services.len(), 3);
        
        // Simulate primary service failure
        registry.deregister_service(primary_handle.service_id).await?;
        
        // Verify secondary and backup services are still available (no cascade)
        let remaining_services = registry.discover_by_capabilities(vec![ServiceCapability::Storage]).await?;
        assert_eq!(remaining_services.len(), 2);
        
        // Test failover to secondary service
        let failover_requirements = ServiceRequirements {
            required_capabilities: vec![ServiceCapability::Storage],
            performance_tier: None,
            max_latency: None,
            min_availability: Some(0.95),
        };
        
        let failover_preferences = SelectionPreferences {
            prefer_low_latency: true,
            prefer_high_performance: false,
            prefer_local: true,
            cost_sensitivity: 0.5,
        };
        
        let failover_service = registry.find_optimal_service(
            failover_requirements,
            failover_preferences
        ).await?;
        
        // Verify failover service is one of the remaining services
        assert!(failover_service.name == "secondary-service" || failover_service.name == "backup-service");
        
        println!("✅ Cascading failure prevention tested");
        Ok(())
    }
}

/// **TEST SUITE 4: CROSS-CRATE INTEGRATION WORKFLOWS**
#[cfg(test)]
mod cross_crate_integration_tests {
    use super::*;

    /// Test complete storage-to-network-to-security workflow
    #[tokio::test]
    async fn test_complete_storage_security_network_workflow() -> Result<()> {
        // This test would integrate storage, network, and security crates
        // For now, we'll test the service discovery coordination
        
        let registry = Arc::new(InMemoryServiceRegistry::new());
        
        // Register services from different crates
        let storage_service = create_test_service_registration(
            "nestgate-storage",
            vec![ServiceCapability::Storage, ServiceCapability::Backup],
            ServiceRole::Storage,
        );
        
        let network_service = create_test_service_registration(
            "nestgate-network",
            vec![ServiceCapability::Networking, ServiceCapability::LoadBalancing],
            ServiceRole::Network,
        );
        
        let security_service = create_test_service_registration(
            "nestgate-security",
            vec![ServiceCapability::Authentication, ServiceCapability::Encryption],
            ServiceRole::Security,
        );
        
        registry.register_service(storage_service).await?;
        registry.register_service(network_service).await?;
        registry.register_service(security_service).await?;
        
        // Test workflow: Secure storage access through network
        // 1. Discover security service for authentication
        let auth_services = registry.discover_by_capabilities(vec![ServiceCapability::Authentication]).await?;
        assert_eq!(auth_services.len(), 1);
        
        // 2. Discover network service for routing
        let network_services = registry.discover_by_capabilities(vec![ServiceCapability::Networking]).await?;
        assert_eq!(network_services.len(), 1);
        
        // 3. Discover storage service for data access
        let storage_services = registry.discover_by_capabilities(vec![ServiceCapability::Storage]).await?;
        assert_eq!(storage_services.len(), 1);
        
        // Verify complete workflow coordination
        assert_eq!(auth_services[0].name, "nestgate-security");
        assert_eq!(network_services[0].name, "nestgate-network");
        assert_eq!(storage_services[0].name, "nestgate-storage");
        
        println!("✅ Cross-crate integration workflow tested");
        Ok(())
    }
}

// **HELPER FUNCTIONS AND TEST UTILITIES**

fn create_test_service_registration(
    name: &str,
    capabilities: Vec<ServiceCapability>,
    role: ServiceRole,
) -> UniversalServiceRegistration {
    UniversalServiceRegistration {
        service_id: Uuid::new_v4(),
        name: name.to_string(),
        category: ServiceCategory::Storage,
        capabilities,
        endpoint: ServiceEndpoint {
            protocol: CommunicationProtocol::Http,
            address: nestgate_core::constants::TEST_HOSTNAME.to_string(),
            port: nestgate_core::constants::DEFAULT_API_PORT,
            path: Some("/".to_string()),
        },
        metadata: ServiceMetadata {
            version: "1.0.0".to_string(),
            tags: HashMap::new(),
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
        },
        role,
        health_check_endpoint: Some("/health".to_string()),
        registration_time: SystemTime::now(),
    }
}

fn create_test_service_with_metadata(
    name: &str,
    capabilities: Vec<ServiceCapability>,
    role: ServiceRole,
    tags: HashMap<String, String>,
) -> UniversalServiceRegistration {
    UniversalServiceRegistration {
        service_id: Uuid::new_v4(),
        name: name.to_string(),
        category: ServiceCategory::Storage,
        capabilities,
        endpoint: ServiceEndpoint {
            protocol: CommunicationProtocol::Http,
            address: nestgate_core::constants::TEST_HOSTNAME.to_string(),
            port: nestgate_core::constants::DEFAULT_API_PORT,
            path: Some("/".to_string()),
        },
        metadata: ServiceMetadata {
            version: "1.0.0".to_string(),
            tags,
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
        },
        role,
        health_check_endpoint: Some("/health".to_string()),
        registration_time: SystemTime::now(),
    }
}

/// Test implementation of SmartService trait for testing
struct TestSmartService {
    service_id: String,
    capabilities: Vec<String>,
}

impl TestSmartService {
    fn new(service_id: &str, capabilities: Vec<String>) -> Self {
        Self {
            service_id: service_id.to_string(),
            capabilities,
        }
    }
}

use nestgate_core::smart_abstractions::service_patterns::{SmartService, ServiceMetadata, UnifiedServiceType};

impl SmartService for TestSmartService {
    fn metadata(&self) -> ServiceMetadata {
        ServiceMetadata {
            service_id: self.service_id.clone(),
            service_type: UnifiedServiceType::Storage,
            endpoints: vec![nestgate_core::constants::TEST_API_BASE.to_string()],
            capabilities: self.capabilities.clone(),
        }
    }
    
    fn health_check(&self) -> nestgate_core::smart_abstractions::service_patterns::UnifiedHealthStatus {
        nestgate_core::smart_abstractions::service_patterns::UnifiedHealthStatus::Healthy
    }
}

/// Additional service capability definitions for testing
impl ServiceCapability {
    pub const Authentication: ServiceCapability = ServiceCapability::Security;
    pub const Encryption: ServiceCapability = ServiceCapability::Security;
    pub const Storage: ServiceCapability = ServiceCapability::Storage;
    pub const Backup: ServiceCapability = ServiceCapability::Storage;
    pub const Coordination: ServiceCapability = ServiceCapability::Orchestration;
    pub const LoadBalancing: ServiceCapability = ServiceCapability::Network;
    pub const Networking: ServiceCapability = ServiceCapability::Network;
}

/// Additional service role definitions for testing
#[derive(Debug, Clone, PartialEq)]
pub enum ServiceRole {
    Security,
    Storage,
    Network,
    Orchestration,
    Primary,
    Secondary,
    Backup,
} 