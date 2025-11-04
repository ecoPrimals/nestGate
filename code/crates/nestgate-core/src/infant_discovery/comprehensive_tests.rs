//! Comprehensive Unit Tests for Infant Discovery Architecture
//!
//! This module provides extensive test coverage for the Infant Discovery system,
//! including capability scanning, connection management, sovereignty validation,
//! and O(1) complexity verification.

use super::*;
use std::time::Duration;

// ==================== INFANT DISCOVERY SYSTEM TESTS ====================

#[tokio::test]
async fn test_infant_discovery_system_creation() {
    let system = InfantDiscoverySystem::<100>::new();
    assert!(
        system.verify_sovereignty_compliance(),
        "System should be sovereignty compliant"
    );
}

#[test]
fn test_default_trait_implementation() {
    let system1 = InfantDiscoverySystem::<50>::new();
    let system2 = InfantDiscoverySystem::<50>::default();

    assert_eq!(
        system1.verify_sovereignty_compliance(),
        system2.verify_sovereignty_compliance()
    );
}

#[tokio::test]
async fn test_capability_discovery_success() {
    let mut system = InfantDiscoverySystem::<100>::new();
    let result = system.discover_capabilities().await;

    assert!(result.is_ok(), "Capability discovery should succeed");
    let capabilities = result.expect("Test setup failed");
    assert!(
        !capabilities.is_empty(),
        "Should discover at least one capability"
    );
}

#[tokio::test]
async fn test_discover_capabilities_sovereignty_compliance() {
    let mut system = InfantDiscoverySystem::<100>::new();

    let capabilities = system
        .discover_capabilities()
        .await
        .expect("Test setup failed");

    // Verify all discovered capabilities are sovereignty compliant
    for cap in &capabilities {
        assert!(
            cap.sovereignty_compliant,
            "Capability {} should be sovereignty compliant",
            cap.id
        );
    }
}

#[tokio::test]
async fn test_establish_connection_success() {
    let mut system = InfantDiscoverySystem::<100>::new();

    // First discover capabilities
    let capabilities = system
        .discover_capabilities()
        .await
        .expect("Test setup failed");
    assert!(
        !capabilities.is_empty(),
        "Should have discovered capabilities"
    );

    // Then establish connection
    let result = system.establish_connection(&capabilities[0].id).await;
    assert!(result.is_ok(), "Connection should succeed");

    let connection = result.expect("Test setup failed");
    assert_eq!(
        connection.id, capabilities[0].id,
        "Connection ID should match capability ID"
    );
    assert_eq!(
        connection.complexity_order, 1,
        "Should maintain O(1) complexity"
    );
}

#[tokio::test]
async fn test_establish_connection_unknown_capability_fails() {
    let mut system = InfantDiscoverySystem::<100>::new();

    let result = system.establish_connection("unknown-capability").await;
    assert!(
        result.is_err(),
        "Connecting to unknown capability should fail"
    );
}

#[tokio::test]
async fn test_o1_complexity_verification() {
    let mut system = InfantDiscoverySystem::<100>::new();

    // Discover capabilities
    let capabilities = system
        .discover_capabilities()
        .await
        .expect("Test setup failed");
    assert!(!capabilities.is_empty());

    // Verify O(1) connection establishment
    let start = std::time::Instant::now();
    let result = system.establish_connection(&capabilities[0].id).await;
    let duration = start.elapsed();

    assert!(result.is_ok(), "Connection should succeed");
    assert!(
        duration < Duration::from_millis(50),
        "Connection should be fast (O(1))"
    );

    let connection = result.expect("Test setup failed");
    assert_eq!(
        connection.complexity_order, 1,
        "Must maintain O(1) complexity order"
    );
}

#[tokio::test]
async fn test_discovery_statistics() {
    let mut system = InfantDiscoverySystem::<100>::new();

    // Perform discovery
    let _capabilities = system
        .discover_capabilities()
        .await
        .expect("Test setup failed");

    // Check statistics
    let stats = system.get_discovery_stats().await;

    assert!(
        stats.total_discovered > 0,
        "Should have discovered capabilities"
    );
    assert!(
        stats.discovery_attempts > 0,
        "Should have recorded discovery attempts"
    );
    assert!(
        stats.avg_discovery_time_ns > 0,
        "Should have recorded discovery time"
    );
}

#[tokio::test]
async fn test_multiple_connections_maintain_o1() {
    let mut system = InfantDiscoverySystem::<100>::new();

    // Discover capabilities
    let capabilities = system
        .discover_capabilities()
        .await
        .expect("Test setup failed");
    assert!(!capabilities.is_empty());

    // Establish multiple connections
    for capability in &capabilities {
        let result = system.establish_connection(&capability.id).await;
        assert!(result.is_ok(), "Each connection should succeed");

        let connection = result.expect("Test setup failed");
        assert_eq!(
            connection.complexity_order, 1,
            "All connections must be O(1)"
        );
    }
}

#[test]
fn test_sovereignty_layer_validates_compliant_capability() {
    let system = InfantDiscoverySystem::<100>::new();

    let capability = CapabilityDescriptor {
        id: "test-capability".to_string(),
        capability_type: CapabilityType::Storage,
        endpoint: Some("test://endpoint".to_string()),
        metadata: HashMap::new(),
        sovereignty_compliant: true,
    };

    assert!(
        system.sovereignty_layer.validate_capability(&capability),
        "Compliant capability should pass validation"
    );
}

#[test]
fn test_sovereignty_layer_rejects_non_compliant_capability() {
    let system = InfantDiscoverySystem::<100>::new();

    let capability = CapabilityDescriptor {
        id: "non-compliant".to_string(),
        capability_type: CapabilityType::Unknown,
        endpoint: Some("test://endpoint".to_string()),
        metadata: HashMap::new(),
        sovereignty_compliant: false,
    };

    assert!(
        !system.sovereignty_layer.validate_capability(&capability),
        "Non-compliant capability should fail validation"
    );
}

#[test]
fn test_sovereignty_layer_rejects_surveillance() {
    let system = InfantDiscoverySystem::<100>::new();

    let mut metadata = HashMap::new();
    metadata.insert("surveillance".to_string(), "enabled".to_string());

    let capability = CapabilityDescriptor {
        id: "surveillance-cap".to_string(),
        capability_type: CapabilityType::Security,
        endpoint: Some("test://endpoint".to_string()),
        metadata,
        sovereignty_compliant: true,
    };

    assert!(
        !system.sovereignty_layer.validate_capability(&capability),
        "Capability with surveillance should be rejected"
    );
}

#[test]
fn test_sovereignty_layer_requires_consent() {
    let system = InfantDiscoverySystem::<100>::new();

    let mut metadata = HashMap::new();
    metadata.insert("consent_required".to_string(), "false".to_string());

    let capability = CapabilityDescriptor {
        id: "no-consent-cap".to_string(),
        capability_type: CapabilityType::Network,
        endpoint: Some("test://endpoint".to_string()),
        metadata,
        sovereignty_compliant: true,
    };

    assert!(
        !system.sovereignty_layer.validate_capability(&capability),
        "Capability without consent should be rejected"
    );
}

// ==================== CAPABILITY DESCRIPTOR TESTS ====================

#[test]
fn test_capability_descriptor_creation() {
    let mut metadata = HashMap::new();
    metadata.insert("key1".to_string(), "value1".to_string());

    let capability = CapabilityDescriptor {
        id: "test-cap".to_string(),
        capability_type: CapabilityType::Storage,
        endpoint: Some("http://example.com".to_string()),
        metadata: metadata.clone(),
        sovereignty_compliant: true,
    };

    assert_eq!(capability.id, "test-cap");
    assert_eq!(capability.capability_type, CapabilityType::Storage);
    assert_eq!(capability.endpoint, Some("http://example.com".to_string()));
    assert_eq!(capability.metadata.len(), 1);
    assert!(capability.sovereignty_compliant);
}

#[test]
fn test_capability_types() {
    let storage_cap = CapabilityDescriptor {
        id: "storage".to_string(),
        capability_type: CapabilityType::Storage,
        endpoint: None,
        metadata: HashMap::new(),
        sovereignty_compliant: true,
    };
    assert_eq!(storage_cap.capability_type, CapabilityType::Storage);

    let compute_cap = CapabilityDescriptor {
        id: "compute".to_string(),
        capability_type: CapabilityType::Compute,
        endpoint: None,
        metadata: HashMap::new(),
        sovereignty_compliant: true,
    };
    assert_eq!(compute_cap.capability_type, CapabilityType::Compute);

    let network_cap = CapabilityDescriptor {
        id: "network".to_string(),
        capability_type: CapabilityType::Network,
        endpoint: None,
        metadata: HashMap::new(),
        sovereignty_compliant: true,
    };
    assert_eq!(network_cap.capability_type, CapabilityType::Network);

    let security_cap = CapabilityDescriptor {
        id: "security".to_string(),
        capability_type: CapabilityType::Security,
        endpoint: None,
        metadata: HashMap::new(),
        sovereignty_compliant: true,
    };
    assert_eq!(security_cap.capability_type, CapabilityType::Security);

    let unknown_cap = CapabilityDescriptor {
        id: "unknown".to_string(),
        capability_type: CapabilityType::Unknown,
        endpoint: None,
        metadata: HashMap::new(),
        sovereignty_compliant: true,
    };
    assert_eq!(unknown_cap.capability_type, CapabilityType::Unknown);
}

#[test]
fn test_capability_without_endpoint() {
    let capability = CapabilityDescriptor {
        id: "no-endpoint".to_string(),
        capability_type: CapabilityType::Compute,
        endpoint: None,
        metadata: HashMap::new(),
        sovereignty_compliant: true,
    };

    assert_eq!(
        capability.endpoint, None,
        "Capability can exist without endpoint"
    );
}

#[test]
fn test_capability_metadata_operations() {
    let mut metadata = HashMap::new();
    metadata.insert("feature1".to_string(), "enabled".to_string());
    metadata.insert("feature2".to_string(), "disabled".to_string());
    metadata.insert("priority".to_string(), "high".to_string());

    let capability = CapabilityDescriptor {
        id: "metadata-test".to_string(),
        capability_type: CapabilityType::Storage,
        endpoint: Some("http://example.com".to_string()),
        metadata: metadata.clone(),
        sovereignty_compliant: true,
    };

    assert_eq!(capability.metadata.len(), 3);
    assert_eq!(
        capability.metadata.get("feature1"),
        Some(&"enabled".to_string())
    );
    assert_eq!(
        capability.metadata.get("priority"),
        Some(&"high".to_string())
    );
}

// ==================== CONNECTION TESTS ====================

#[tokio::test]
async fn test_connection_creation() {
    let mut system = InfantDiscoverySystem::<100>::new();
    let capabilities = system
        .discover_capabilities()
        .await
        .expect("Test setup failed");
    let connection = system
        .establish_connection(&capabilities[0].id)
        .await
        .expect("Test setup failed");

    assert_eq!(connection.id, capabilities[0].id);
    assert_eq!(connection.complexity_order, 1, "Must be O(1)");
}

#[tokio::test]
async fn test_connection_timestamp() {
    let mut system = InfantDiscoverySystem::<100>::new();
    let capabilities = system
        .discover_capabilities()
        .await
        .expect("Test setup failed");

    let before = std::time::SystemTime::now();
    let connection = system
        .establish_connection(&capabilities[0].id)
        .await
        .expect("Test setup failed");
    let after = std::time::SystemTime::now();

    assert!(
        connection.established_at >= before,
        "Connection timestamp should be after 'before'"
    );
    assert!(
        connection.established_at <= after,
        "Connection timestamp should be before 'after'"
    );
}

#[tokio::test]
async fn test_connection_endpoint_matches_capability() {
    let mut system = InfantDiscoverySystem::<100>::new();
    let capabilities = system
        .discover_capabilities()
        .await
        .expect("Test setup failed");

    let connection = system
        .establish_connection(&capabilities[0].id)
        .await
        .expect("Test setup failed");

    assert_eq!(
        connection.endpoint, capabilities[0].endpoint,
        "Connection endpoint should match capability endpoint"
    );
}

// ==================== DISCOVERY STATISTICS TESTS ====================

#[tokio::test]
async fn test_discovery_stats_default() {
    let stats = DiscoveryStats::default();
    assert_eq!(stats.total_discovered, 0);
    assert_eq!(stats.discovery_attempts, 0);
    assert_eq!(stats.avg_discovery_time_ns, 0);
}

#[tokio::test]
async fn test_discovery_stats_tracking() {
    let mut system = InfantDiscoverySystem::<100>::new();

    // Initial stats
    let initial_stats = system.get_discovery_stats().await;
    assert_eq!(initial_stats.discovery_attempts, 0);

    // Perform discovery
    let _capabilities = system
        .discover_capabilities()
        .await
        .expect("Test setup failed");

    // Updated stats
    let updated_stats = system.get_discovery_stats().await;
    assert_eq!(
        updated_stats.discovery_attempts, 1,
        "Should have recorded 1 discovery attempt"
    );
    assert!(
        updated_stats.total_discovered > 0,
        "Should have discovered capabilities"
    );
    assert!(
        updated_stats.avg_discovery_time_ns > 0,
        "Should have recorded discovery time"
    );
}

#[tokio::test]
async fn test_discovery_stats_multiple_discoveries() {
    let mut system = InfantDiscoverySystem::<100>::new();

    // Perform multiple discoveries
    for _ in 0..3 {
        let _capabilities = system
            .discover_capabilities()
            .await
            .expect("Test setup failed");
    }

    let stats = system.get_discovery_stats().await;
    assert_eq!(
        stats.discovery_attempts, 3,
        "Should have recorded 3 discovery attempts"
    );
}

// ==================== INTEGRATION TESTS ====================

#[tokio::test]
async fn test_end_to_end_discovery_and_connection() {
    let mut system = InfantDiscoverySystem::<100>::new();

    // Step 1: Discover capabilities
    let capabilities = system.discover_capabilities().await;
    assert!(capabilities.is_ok(), "Discovery should succeed");

    let caps = capabilities.expect("Test setup failed");
    assert!(!caps.is_empty(), "Should discover at least one capability");

    // Step 2: Establish connection
    let connection = system.establish_connection(&caps[0].id).await;
    assert!(connection.is_ok(), "Connection should succeed");

    let conn = connection.expect("Test setup failed");
    assert_eq!(conn.id, caps[0].id);
    assert_eq!(conn.endpoint, caps[0].endpoint);
    assert_eq!(conn.complexity_order, 1);
}

#[tokio::test]
async fn test_discovery_multiple_times() {
    let mut system = InfantDiscoverySystem::<100>::new();

    // Discover multiple times
    let caps1 = system
        .discover_capabilities()
        .await
        .expect("Test setup failed");
    let caps2 = system
        .discover_capabilities()
        .await
        .expect("Test setup failed");

    // Both should succeed
    assert!(!caps1.is_empty());
    assert!(!caps2.is_empty());

    // Stats should reflect multiple discoveries
    let stats = system.get_discovery_stats().await;
    assert_eq!(stats.discovery_attempts, 2);
}

#[tokio::test]
async fn test_sovereignty_compliance_end_to_end() {
    let mut system = InfantDiscoverySystem::<100>::new();

    // Verify system is compliant
    assert!(system.verify_sovereignty_compliance());

    // Discover capabilities
    let capabilities = system
        .discover_capabilities()
        .await
        .expect("Test setup failed");

    // All discovered capabilities should be sovereignty compliant
    for cap in &capabilities {
        assert!(cap.sovereignty_compliant);
        assert!(system.sovereignty_layer.validate_capability(cap));
    }
}
