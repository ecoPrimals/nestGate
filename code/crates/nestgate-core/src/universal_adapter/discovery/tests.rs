// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use super::*;
use crate::canonical_types::service::{ServiceState, ServiceType};
use crate::config::discovery_config::ServiceDiscoveryConfig;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

/// Helper to create test endpoint using ServiceDiscoveryConfig
/// ✅ MIGRATED: Replaces hardcoded "localhost:port" with configurable endpoints
fn test_endpoint(port: u16) -> String {
    let config = ServiceDiscoveryConfig::default();
    format!("http://{}:{}", config.discovery_host, port)
}

#[test]
fn test_discovery_config_default() {
    let config = DiscoveryConfig::default();

    assert_eq!(config.max_retries, 3);
    assert_eq!(config.timeout, Duration::from_secs(30));
    assert_eq!(config.discovery_interval, Duration::from_secs(60));
    assert_eq!(config.methods.len(), 2);
    assert!(config.methods.contains(&DiscoveryMethod::Environment));
    assert!(config.methods.contains(&DiscoveryMethod::ServiceRegistry));
}

#[test]
fn test_discovery_method_equality() {
    assert_eq!(DiscoveryMethod::Environment, DiscoveryMethod::Environment);
    assert_ne!(
        DiscoveryMethod::Environment,
        DiscoveryMethod::ServiceRegistry
    );
    assert_ne!(DiscoveryMethod::NetworkScan, DiscoveryMethod::Dns);
}

#[test]
fn test_discovery_method_clone() {
    let method = DiscoveryMethod::Configuration;
    let cloned = method.clone();
    assert_eq!(method, cloned);
}

#[test]
fn test_discovered_service_creation() {
    use crate::constants::{network_defaults::LOCALHOST_NAME, port_defaults::get_admin_port};
    let endpoint = format!("http://{}:{}", LOCALHOST_NAME, get_admin_port());
    let service = DiscoveredService {
        id: "test-service-1".to_string(),
        name: "Test Service".to_string(),
        service_type: ServiceType::Storage,
        state: ServiceState::Running,
        endpoint,
        capabilities: vec!["storage".to_string(), "backup".to_string()],
        metadata: HashMap::new(),
        discovered_at: SystemTime::now(),
        last_health_check: Some(SystemTime::now()),
    };

    assert_eq!(service.id, "test-service-1");
    assert_eq!(service.name, "Test Service");
    assert_eq!(service.capabilities.len(), 2);
    assert!(service.last_health_check.is_some());
}

#[test]
fn test_discovery_result_success() {
    let result = DiscoveryResult {
        services: vec![],
        method: DiscoveryMethod::Environment,
        duration: Duration::from_millis(100),
        success: true,
        error: None,
    };

    assert!(result.success);
    assert!(result.error.is_none());
    assert_eq!(result.method, DiscoveryMethod::Environment);
}

#[test]
fn test_discovery_result_failure() {
    let result = DiscoveryResult {
        services: vec![],
        method: DiscoveryMethod::ServiceRegistry,
        duration: Duration::from_millis(50),
        success: false,
        error: Some("Connection timeout".to_string()),
    };

    assert!(!result.success);
    assert!(result.error.is_some());
    assert_eq!(
        result.error.expect("Operation failed"),
        "Connection timeout"
    );
}

#[test]
fn test_discover_services() {
    let config = DiscoveryConfig::default();
    let result = discover_services(&config).expect("Operation failed");

    assert!(result.success);
    assert!(result.error.is_none());
    assert_eq!(result.services.len(), 1);
    assert_eq!(result.services[0].name, "NestGate Core");
}

#[test]
fn test_health_check_running_service() {
    use crate::constants::{network_defaults::LOCALHOST_NAME, port_defaults::get_admin_port};
    let endpoint = format!("http://{}:{}", LOCALHOST_NAME, get_admin_port());
    let service = DiscoveredService {
        id: "test-1".to_string(),
        name: "Test".to_string(),
        service_type: ServiceType::Storage,
        state: ServiceState::Running,
        endpoint,
        capabilities: vec![],
        metadata: HashMap::new(),
        discovered_at: SystemTime::now(),
        last_health_check: None,
    };

    let healthy = health_check_service(&service);
    assert!(healthy);
}

#[test]
fn test_health_check_stopped_service() {
    let service = DiscoveredService {
        id: "test-2".to_string(),
        name: "Test".to_string(),
        service_type: ServiceType::Storage,
        state: ServiceState::Stopped,
        endpoint: test_endpoint(8080),
        capabilities: vec![],
        metadata: HashMap::new(),
        discovered_at: SystemTime::now(),
        last_health_check: None,
    };

    let healthy = health_check_service(&service);
    assert!(!healthy);
}

#[test]
fn test_health_check_starting_service() {
    let service = DiscoveredService {
        id: "test-3".to_string(),
        name: "Test".to_string(),
        service_type: ServiceType::Storage,
        state: ServiceState::Starting,
        endpoint: test_endpoint(8080),
        capabilities: vec![],
        metadata: HashMap::new(),
        discovered_at: SystemTime::now(),
        last_health_check: None,
    };

    let healthy = health_check_service(&service);
    assert!(!healthy);
}

#[test]
fn test_health_check_failed_service() {
    let service = DiscoveredService {
        id: "test-4".to_string(),
        name: "Test".to_string(),
        service_type: ServiceType::Storage,
        state: ServiceState::Failed,
        endpoint: test_endpoint(8080),
        capabilities: vec![],
        metadata: HashMap::new(),
        discovered_at: SystemTime::now(),
        last_health_check: None,
    };

    let healthy = health_check_service(&service);
    assert!(!healthy);
}

#[test]
fn test_capability_discovery_new() {
    let discovery = CapabilityDiscovery::new().expect("Operation failed");

    // ✅ MIGRATED: ServiceDiscoveryConfig generates 3 endpoints by default (port range of 3)
    assert_eq!(discovery.discovery_endpoints.len(), 3);
    assert!(discovery.registry.contains_key("security"));
    assert!(discovery.registry.contains_key("ai"));
    assert!(discovery.registry.contains_key("orchestration"));
    assert!(discovery.registry.contains_key("storage"));
    assert!(discovery.registry.contains_key("compute"));
}

#[test]
fn test_capability_discovery_default() {
    let discovery = CapabilityDiscovery::default();

    assert!(!discovery.registry.is_empty());
    // ✅ MIGRATED: ServiceDiscoveryConfig generates 3 endpoints by default
    assert_eq!(discovery.discovery_endpoints.len(), 3);
}

#[test]
fn test_find_capabilities_existing() {
    let discovery = CapabilityDiscovery::new().expect("Operation failed");
    let capabilities = discovery
        .find_capabilities("security")
        .expect("Operation failed");

    assert!(!capabilities.is_empty());
    assert!(capabilities[0].contains("security"));
}

#[test]
fn test_find_capabilities_nonexistent() {
    let discovery = CapabilityDiscovery::new().expect("Operation failed");
    let capabilities = discovery
        .find_capabilities("nonexistent")
        .expect("Operation failed");

    assert!(capabilities.is_empty());
}

#[test]
fn test_register_capability() {
    let mut discovery = CapabilityDiscovery::new().expect("Operation failed");

    let custom_endpoint = test_endpoint(9000) + "/custom";
    discovery.register_capability("custom".to_string(), custom_endpoint.clone());

    let capabilities = discovery
        .find_capabilities("custom")
        .expect("Operation failed");
    assert_eq!(capabilities.len(), 1);
    assert_eq!(capabilities[0], custom_endpoint);
}

#[test]
fn test_register_multiple_capabilities() {
    let mut discovery = CapabilityDiscovery::new().expect("Operation failed");

    discovery.register_capability("custom".to_string(), test_endpoint(9000) + "/custom1");
    discovery.register_capability("custom".to_string(), test_endpoint(9001) + "/custom2");

    let capabilities = discovery
        .find_capabilities("custom")
        .expect("Operation failed");
    assert_eq!(capabilities.len(), 2);
}

#[test]
fn test_unregister_capability() {
    let mut discovery = CapabilityDiscovery::new().expect("Operation failed");

    let temp_endpoint = test_endpoint(9000) + "/temp";
    discovery.register_capability("temp".to_string(), temp_endpoint.clone());

    let before = discovery
        .find_capabilities("temp")
        .expect("Operation failed");
    assert_eq!(before.len(), 1);

    discovery.unregister_capability("temp", &temp_endpoint);

    let after = discovery
        .find_capabilities("temp")
        .expect("Operation failed");
    assert_eq!(after.len(), 0);
}

#[test]
fn test_unregister_nonexistent_capability() {
    let mut discovery = CapabilityDiscovery::new().expect("Operation failed");

    // Should not panic
    let nonexistent = test_endpoint(9000);
    discovery.unregister_capability("nonexistent", &nonexistent);
}

#[test]
fn test_discovery_config_custom() {
    let config = DiscoveryConfig {
        endpoint: "http://custom:9000".to_string(),
        timeout: Duration::from_secs(60),
        max_retries: 5,
        discovery_interval: Duration::from_secs(120),
        methods: vec![
            DiscoveryMethod::Environment,
            DiscoveryMethod::NetworkScan,
            DiscoveryMethod::Dns,
        ],
    };

    assert_eq!(config.endpoint, "http://custom:9000");
    assert_eq!(config.max_retries, 5);
    assert_eq!(config.methods.len(), 3);
}

#[test]
fn test_discovered_service_with_metadata() {
    let mut metadata = HashMap::new();
    metadata.insert("version".to_string(), "1.0.0".to_string());
    metadata.insert("region".to_string(), "us-west".to_string());

    let service = DiscoveredService {
        id: "test-meta".to_string(),
        name: "Test Meta Service".to_string(),
        service_type: ServiceType::Compute,
        state: ServiceState::Running,
        endpoint: test_endpoint(8080),
        capabilities: vec!["compute".to_string()],
        metadata,
        discovered_at: SystemTime::now(),
        last_health_check: Some(SystemTime::now()),
    };

    assert_eq!(service.metadata.len(), 2);
    assert_eq!(service.metadata.get("version"), Some(&"1.0.0".to_string()));
    assert_eq!(service.metadata.get("region"), Some(&"us-west".to_string()));
}

#[test]
fn test_discovery_result_clone() {
    let result = DiscoveryResult {
        services: vec![],
        method: DiscoveryMethod::Configuration,
        duration: Duration::from_millis(75),
        success: true,
        error: None,
    };

    let cloned = result.clone();
    assert_eq!(cloned.success, result.success);
    assert_eq!(cloned.method, result.method);
}
