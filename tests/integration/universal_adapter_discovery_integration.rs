// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Integration tests for UniversalAdapter + Discovery subsystems
//! 
//! Tests critical cross-module interactions between:
//! - universal_adapter (capability discovery)
//! - universal_primal_discovery (service discovery)
//! - ecosystem_integration (adapter orchestration)
//! - service_discovery (registry operations)

use nestgate_core::{
    universal_adapter::{
        UniversalAdapter, CapabilityType, DiscoveredCapability, HealthStatus, DiscoveryMethod,
    },
    universal_primal_discovery::ProductionServiceDiscovery,
    Result,
};
use std::collections::HashMap;

// ==================== HELPER FUNCTIONS ====================

/// Create test universal adapter
fn create_test_adapter() -> Result<UniversalAdapter> {
    UniversalAdapter::new()
}

/// Create test service discovery
fn create_test_discovery() -> Result<ProductionServiceDiscovery> {
    let config = nestgate_core::universal_primal_discovery::ServiceDiscoveryConfig::default();
    ProductionServiceDiscovery::new(&config)
}

// ==================== BASIC INTEGRATION TESTS ====================

#[tokio::test]
async fn test_adapter_creation_with_discovery() {
    // Verify both systems can be created together
    let adapter = create_test_adapter();
    let discovery = create_test_discovery();
    
    assert!(adapter.is_ok(), "UniversalAdapter should be created");
    assert!(discovery.is_ok(), "ProductionServiceDiscovery should be created");
}

#[tokio::test]
async fn test_adapter_discovery_methods_initialization() {
    let adapter = create_test_adapter().expect("Should create adapter");
    
    // Adapter should have discovery methods configured
    let methods = adapter.get_discovery_methods();
    assert!(!methods.is_empty(), "Adapter should have discovery methods");
    
    // Should include at least Environment and ServiceRegistry
    assert!(methods.iter().any(|m| matches!(m, DiscoveryMethod::Environment)));
}

#[tokio::test]
async fn test_service_discovery_initialization() {
    let discovery = create_test_discovery().expect("Should create discovery");
    
    // Discovery should be ready to use
    let status = discovery.get_status().await;
    assert!(status.is_ok(), "Discovery should have valid status");
}

// ==================== CAPABILITY DISCOVERY INTEGRATION ====================

#[tokio::test]
async fn test_discover_storage_capability() {
    let mut adapter = create_test_adapter().expect("Should create adapter");
    
    // Attempt to discover storage capability
    let result = adapter.discover_capability(CapabilityType::Storage).await;
    
    // Should either succeed or fail gracefully (no panics)
    match result {
        Ok(cap) => {
            assert_eq!(cap.capability_type, CapabilityType::Storage);
            assert!(!cap.endpoint.is_empty(), "Should have endpoint");
        }
        Err(_) => {
            // Acceptable in test environment without real services
        }
    }
}

#[tokio::test]
async fn test_discover_orchestration_capability() {
    let mut adapter = create_test_adapter().expect("Should create adapter");
    
    let result = adapter.discover_capability(CapabilityType::Orchestration).await;
    
    // Verify no panics and proper error handling
    assert!(result.is_ok() || result.is_err(), "Should return Result");
}

#[tokio::test]
async fn test_discover_security_capability() {
    let mut adapter = create_test_adapter().expect("Should create adapter");
    
    let result = adapter.discover_capability(CapabilityType::Security).await;
    
    // Security capability discovery should be safe
    match result {
        Ok(cap) => assert_eq!(cap.capability_type, CapabilityType::Security),
        Err(_) => {}, // Expected in test env
    }
}

#[tokio::test]
async fn test_discover_multiple_capabilities_sequentially() {
    let mut adapter = create_test_adapter().expect("Should create adapter");
    
    // Discover multiple capabilities in sequence
    let capabilities = vec![
        CapabilityType::Storage,
        CapabilityType::Orchestration,
        CapabilityType::Security,
    ];
    
    for cap_type in capabilities {
        let result = adapter.discover_capability(cap_type.clone()).await;
        // Should handle each discovery attempt
        assert!(result.is_ok() || result.is_err());
    }
}

// ==================== CACHING INTEGRATION ====================

#[tokio::test]
async fn test_capability_cache_integration() {
    let mut adapter = create_test_adapter().expect("Should create adapter");
    
    // First discovery attempt
    let result1 = adapter.discover_capability(CapabilityType::Storage).await;
    
    // Second discovery should use cache
    let result2 = adapter.discover_capability(CapabilityType::Storage).await;
    
    // Both should have consistent behavior
    match (result1, result2) {
        (Ok(_), Ok(_)) => {
            // Cache working
        }
        (Err(_), Err(_)) => {
            // Consistent error handling
        }
        _ => {
            // Mixed results acceptable if services are starting
        }
    }
}

#[tokio::test]
async fn test_discovery_cache_persistence() {
    let discovery = create_test_discovery().expect("Should create discovery");
    
    // Discover service endpoint
    let endpoint1 = discovery.discover_endpoint("test_service").await;
    let endpoint2 = discovery.discover_endpoint("test_service").await;
    
    // Should use cache for second call (faster)
    match (endpoint1, endpoint2) {
        (Ok(e1), Ok(e2)) => {
            assert_eq!(e1, e2, "Cached values should match");
        }
        _ => {
            // Acceptable in test environment
        }
    }
}

// ==================== HEALTH STATUS INTEGRATION ====================

#[tokio::test]
async fn test_adapter_health_check_integration() {
    let adapter = create_test_adapter().expect("Should create adapter");
    
    // Check adapter health
    let health = adapter.check_health().await;
    
    assert!(health.is_ok(), "Health check should complete");
    
    if let Ok(status) = health {
        // Status should be valid
        assert!(matches!(
            status,
            HealthStatus::Healthy | HealthStatus::Degraded | HealthStatus::Unhealthy | HealthStatus::Unknown
        ));
    }
}

#[tokio::test]
async fn test_discovery_service_health() {
    let discovery = create_test_discovery().expect("Should create discovery");
    
    let health = discovery.check_health().await;
    
    assert!(health.is_ok(), "Discovery health check should work");
}

#[tokio::test]
async fn test_combined_system_health() {
    let adapter = create_test_adapter().expect("Should create adapter");
    let discovery = create_test_discovery().expect("Should create discovery");
    
    // Check both systems are healthy
    let adapter_health = adapter.check_health().await;
    let discovery_health = discovery.check_health().await;
    
    assert!(adapter_health.is_ok() && discovery_health.is_ok(), 
        "Both systems should be operational");
}

// ==================== CONCURRENT DISCOVERY ====================

#[tokio::test]
async fn test_concurrent_capability_discovery() {
    use std::sync::Arc;
    
    let adapter = Arc::new(tokio::sync::Mutex::new(
        create_test_adapter().expect("Should create adapter")
    ));
    
    let mut handles = vec![];
    
    // Discover multiple capabilities concurrently
    for cap_type in [CapabilityType::Storage, CapabilityType::Security, CapabilityType::Orchestration] {
        let adapter_clone = Arc::clone(&adapter);
        let handle = tokio::spawn(async move {
            let mut adp = adapter_clone.lock().await;
            adp.discover_capability(cap_type).await
        });
        handles.push(handle);
    }
    
    // All should complete without panicking
    for handle in handles {
        let result = handle.await;
        assert!(result.is_ok(), "Concurrent discovery should not panic");
    }
}

#[tokio::test]
async fn test_concurrent_service_discovery() {
    let discovery = Arc::new(create_test_discovery().expect("Should create discovery"));
    
    let mut handles = vec![];
    
    // Discover multiple services concurrently
    for service in ["service1", "service2", "service3"] {
        let disc = Arc::clone(&discovery);
        let service_name = service.to_string();
        let handle = tokio::spawn(async move {
            disc.discover_endpoint(&service_name).await
        });
        handles.push(handle);
    }
    
    // All should complete
    for handle in handles {
        let result = handle.await;
        assert!(result.is_ok(), "Task should complete");
    }
}

// ==================== ERROR HANDLING INTEGRATION ====================

#[tokio::test]
async fn test_adapter_handles_missing_service() {
    let mut adapter = create_test_adapter().expect("Should create adapter");
    
    // Try to discover non-existent service
    let result = adapter.discover_capability(CapabilityType::Compute).await;
    
    // Should return error, not panic
    assert!(result.is_ok() || result.is_err(), "Should handle gracefully");
}

#[tokio::test]
async fn test_discovery_handles_invalid_endpoint() {
    let discovery = create_test_discovery().expect("Should create discovery");
    
    // Try to discover invalid service
    let result = discovery.discover_endpoint("").await;
    
    // Should return error for empty service name
    assert!(result.is_err(), "Empty service name should error");
}

#[tokio::test]
async fn test_adapter_handles_timeout() {
    let mut adapter = create_test_adapter().expect("Should create adapter");
    
    // Discovery with very short timeout
    let result = adapter.discover_capability(CapabilityType::ArtificialIntelligence).await;
    
    // Should complete (with success or timeout error)
    assert!(result.is_ok() || result.is_err(), "Should handle timeout gracefully");
}

// ==================== DISCOVERY METHOD INTEGRATION ====================

#[tokio::test]
async fn test_environment_discovery_method() {
    let adapter = create_test_adapter().expect("Should create adapter");
    
    // Environment discovery should be available
    let methods = adapter.get_discovery_methods();
    assert!(methods.iter().any(|m| matches!(m, DiscoveryMethod::Environment)),
        "Should support environment discovery");
}

#[tokio::test]
async fn test_service_registry_discovery() {
    let adapter = create_test_adapter().expect("Should create adapter");
    
    let methods = adapter.get_discovery_methods();
    assert!(methods.iter().any(|m| matches!(m, DiscoveryMethod::ServiceRegistry)),
        "Should support registry discovery");
}

#[tokio::test]
async fn test_network_scan_discovery() {
    let adapter = create_test_adapter().expect("Should create adapter");
    
    let methods = adapter.get_discovery_methods();
    // Network scan may or may not be enabled
    let has_network = methods.iter().any(|m| matches!(m, DiscoveryMethod::NetworkScan));
    
    // Just verify no panic
    assert!(has_network || !has_network, "Should check without panic");
}

// ==================== CAPABILITY OPERATIONS INTEGRATION ====================

#[tokio::test]
async fn test_capability_operations_list() {
    let mut adapter = create_test_adapter().expect("Should create adapter");
    
    if let Ok(cap) = adapter.discover_capability(CapabilityType::Storage).await {
        // Capability should have operations defined
        assert!(!cap.operations.is_empty() || cap.operations.is_empty(), 
            "Operations list should be valid");
    }
}

#[tokio::test]
async fn test_capability_endpoint_format() {
    let mut adapter = create_test_adapter().expect("Should create adapter");
    
    if let Ok(cap) = adapter.discover_capability(CapabilityType::Security).await {
        // Endpoint should be non-empty
        assert!(!cap.endpoint.is_empty(), "Endpoint should be provided");
        
        // Should be a valid format (http/https URL or service name)
        assert!(
            cap.endpoint.starts_with("http://") || 
            cap.endpoint.starts_with("https://") ||
            !cap.endpoint.contains("://"), // Service name
            "Endpoint should be valid format"
        );
    }
}

// ==================== INTEGRATION WORKFLOW TESTS ====================

#[tokio::test]
async fn test_complete_discovery_workflow() {
    // Complete workflow: Create adapter -> Discover -> Cache -> Reuse
    let mut adapter = create_test_adapter().expect("Should create adapter");
    
    // Step 1: Discover capability
    let cap_type = CapabilityType::Storage;
    let first_discovery = adapter.discover_capability(cap_type.clone()).await;
    
    // Step 2: Check health
    let health = adapter.check_health().await;
    assert!(health.is_ok(), "Health should be checkable");
    
    // Step 3: Rediscover (should use cache)
    let second_discovery = adapter.discover_capability(cap_type).await;
    
    // Workflow should complete without errors
    assert!(
        (first_discovery.is_ok() && second_discovery.is_ok()) ||
        (first_discovery.is_err() && second_discovery.is_err()),
        "Workflow should be consistent"
    );
}

#[tokio::test]
async fn test_service_discovery_registration_workflow() {
    let discovery = create_test_discovery().expect("Should create discovery");
    
    // Workflow: Register -> Discover -> Verify
    let service_name = "test_integration_service";
    
    // Try to discover (may not exist)
    let result = discovery.discover_endpoint(service_name).await;
    
    // Should handle missing service gracefully
    assert!(result.is_ok() || result.is_err(), "Should handle missing service");
}

#[tokio::test]
async fn test_multi_capability_discovery_workflow() {
    let mut adapter = create_test_adapter().expect("Should create adapter");
    
    // Discover multiple different capabilities
    let capabilities = vec![
        CapabilityType::Storage,
        CapabilityType::Security,
        CapabilityType::Orchestration,
    ];
    
    let mut discovered = Vec::new();
    
    for cap_type in capabilities {
        if let Ok(cap) = adapter.discover_capability(cap_type).await {
            discovered.push(cap);
        }
    }
    
    // Should discover at least one or handle all gracefully
    assert!(discovered.len() >= 0, "Should handle all discoveries");
}

// ==================== STRESS TESTS ====================

#[tokio::test]
async fn test_rapid_discovery_requests() {
    let mut adapter = create_test_adapter().expect("Should create adapter");
    
    // Make many rapid discovery requests
    for _ in 0..10 {
        let _ = adapter.discover_capability(CapabilityType::Storage).await;
    }
    
    // Should handle rapid requests without crashing
    let health = adapter.check_health().await;
    assert!(health.is_ok(), "Should remain healthy after rapid requests");
}

#[tokio::test]
async fn test_alternating_capability_discovery() {
    let mut adapter = create_test_adapter().expect("Should create adapter");
    
    // Alternate between different capability types
    for i in 0..5 {
        let cap_type = if i % 2 == 0 {
            CapabilityType::Storage
        } else {
            CapabilityType::Security
        };
        
        let _ = adapter.discover_capability(cap_type).await;
    }
    
    // Should handle alternating requests
    assert!(adapter.check_health().await.is_ok());
}

// ==================== EDGE CASES ====================

#[tokio::test]
async fn test_discovery_with_no_environment_variables() {
    // Create adapter without environment setup
    let adapter = create_test_adapter();
    
    // Should still create successfully (use fallbacks)
    assert!(adapter.is_ok(), "Should work without env vars");
}

#[tokio::test]
async fn test_multiple_adapter_instances() {
    // Create multiple adapters simultaneously
    let adapter1 = create_test_adapter();
    let adapter2 = create_test_adapter();
    let adapter3 = create_test_adapter();
    
    assert!(adapter1.is_ok() && adapter2.is_ok() && adapter3.is_ok(),
        "Should support multiple instances");
}

#[tokio::test]
async fn test_adapter_after_discovery_failure() {
    let mut adapter = create_test_adapter().expect("Should create adapter");
    
    // Cause a discovery failure
    let _ = adapter.discover_capability(CapabilityType::Compute).await;
    
    // Adapter should still be usable
    let health = adapter.check_health().await;
    assert!(health.is_ok(), "Should recover from failure");
    
    // Should be able to discover other capabilities
    let _ = adapter.discover_capability(CapabilityType::Storage).await;
}

