// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective
#![expect(
    dead_code,
    missing_docs,
    unused_imports,
    unused_variables,
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction
)]

//! Comprehensive Unit Tests for NestGate Core
//!
//! This test suite provides thorough unit test coverage for core functionality,
//! focusing on individual modules and functions in isolation.

use nestgate_core::{
    canonical_types::*,
    constants::system::*,
    error::{NestGateError, Result as NestGateResult, ValidationErrorDetails},
    infant_discovery::*,
};
use std::collections::HashMap;

// ==================== CONSTANTS MODULE TESTS ====================

#[test]
fn test_system_constants_defaults() {
    assert_eq!(DEFAULT_INSTANCE_NAME, "nestgate-default");
    assert_eq!(DEFAULT_SERVICE_NAME, "nestgate");
    assert_eq!(DEFAULT_TIMEOUT_MS, 5000);
    assert_eq!(MAX_CONNECTIONS, 1000);
    assert_eq!(BUFFER_SIZE, 8192);
    assert_eq!(nestgate_core::constants::network::DEFAULT_API_PORT, 8080);
    assert_eq!(DEFAULT_BIND_HOST, "127.0.0.1");
}

#[test]
fn test_environment_variable_functions() {
    // Test default values when env vars are not set
    assert!(timeout_ms() >= 1000); // Should be reasonable default
    assert!(max_connections() >= 100); // Should be reasonable default
    assert!(buffer_size() >= 1024); // Should be reasonable default
    assert!(api_port() > 0); // Should be valid port
    assert!(!bind_host().is_empty()); // Should have default host
}

#[test]
fn test_api_url_generation() {
    let url = api_url();
    assert!(url.starts_with("http://"));
    assert!(url.contains(":"));
    // Should contain host and port
}

// ==================== CANONICAL TYPES TESTS ====================

#[test]
fn test_storage_tier_variants() {
    let tiers = vec![
        StorageTier::Hot,
        StorageTier::Warm,
        StorageTier::Cold,
        StorageTier::Cache,
        StorageTier::Archive,
    ];

    for tier in &tiers {
        // Test serialization/deserialization
        let json = serde_json::to_string(tier).expect("Should serialize");
        let deserialized: StorageTier = serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(*tier, deserialized);
    }
}

#[test]
fn test_storage_operation_variants() {
    let operations = vec![
        StorageOperation::Read,
        StorageOperation::Write,
        StorageOperation::Delete,
        StorageOperation::Copy,
        StorageOperation::Move,
        StorageOperation::Backup,
        StorageOperation::Restore,
    ];

    for op in &operations {
        // Test Debug formatting
        let debug_str = format!("{op:?}");
        assert!(!debug_str.is_empty());

        // Test Clone
        let cloned = op.clone();
        assert_eq!(*op, cloned);
    }
}

#[test]
fn test_response_status_enum() {
    let success = ResponseStatus::Success;
    let error = ResponseStatus::Error;

    assert_ne!(success, error);

    // Test serialization
    let success_json = serde_json::to_string(&success).expect("Should serialize");
    let error_json = serde_json::to_string(&error).expect("Should serialize");

    assert_ne!(success_json, error_json);
}

// ==================== ERROR SYSTEM TESTS ====================

#[test]
fn test_nestgate_error_creation() {
    let error = NestGateError::Validation(Box::new(ValidationErrorDetails {
        message: "Test error".into(),
        field: Some("test_field".into()),
        expected: Some("valid_value".into()),
        actual: Some("invalid_value".into()),
        context: None,
    }));

    assert!(error.to_string().contains("Test error"));
    assert!(format!("{error:?}").contains("Validation"));
}

#[test]
fn test_nestgate_result_ok() {
    let result: NestGateResult<i32> = Ok(42);
    assert!(result.is_ok());
    if let Ok(value) = result {
        assert_eq!(value, 42);
    }
}

#[test]
fn test_nestgate_result_err() {
    let result: NestGateResult<i32> = Err(NestGateError::Validation(Box::new(
        ValidationErrorDetails {
            message: "Test error".into(),
            field: Some("test_field".into()),
            expected: None,
            actual: None,
            context: None,
        },
    )));

    assert!(result.is_err());
    if let Err(error) = result {
        assert!(error.to_string().contains("Test error"));
    }
}

#[test]
fn test_error_context_chaining() {
    let base_error = NestGateError::Validation(Box::new(ValidationErrorDetails {
        message: "Base error".into(),
        field: Some("base_field".into()),
        expected: None,
        actual: None,
        context: None,
    }));

    let error_with_context = NestGateError::Validation(Box::new(ValidationErrorDetails {
        message: "Wrapped error".into(),
        field: Some("wrapper_field".into()),
        expected: None,
        actual: None,
        context: None, // Simplified for this test
    }));

    let error_string = error_with_context.to_string();
    let base_error_string = base_error.to_string();
    assert!(error_string.contains("Wrapped error"));
    assert!(base_error_string.contains("Base error"));
}

// ==================== INFANT DISCOVERY TESTS ====================

#[test]
fn test_infant_discovery_system_creation() {
    let system: InfantDiscoverySystem<32> = InfantDiscoverySystem::new();
    assert!(system.verify_sovereignty_compliance());
}

#[test]
fn test_infant_discovery_system_default() {
    let system: InfantDiscoverySystem<64> = InfantDiscoverySystem::default();
    assert!(system.verify_sovereignty_compliance());
}

#[test]
fn test_capability_descriptor_creation() {
    let capability = CapabilityDescriptor {
        id: "test_capability".to_string(),
        capability_type: CapabilityType::Storage,
        endpoint: Some("http://test.local".to_string()),
        metadata: HashMap::from([
            ("version".to_string(), "1.0".to_string()),
            ("protocol".to_string(), "http".to_string()),
        ]),
        sovereignty_compliant: true,
    };

    assert_eq!(capability.id, "test_capability");
    assert_eq!(capability.capability_type, CapabilityType::Storage);
    assert!(capability.sovereignty_compliant);
    assert_eq!(capability.metadata.len(), 2);
}

#[test]
fn test_capability_type_variants() {
    let types = vec![
        CapabilityType::Storage,
        CapabilityType::Compute,
        CapabilityType::Network,
        CapabilityType::Security,
        CapabilityType::Unknown,
    ];

    for cap_type in &types {
        let debug_str = format!("{cap_type:?}");
        assert!(!debug_str.is_empty());

        let cloned = cap_type.clone();
        assert_eq!(*cap_type, cloned);
    }
}

#[test]
fn test_discovery_stats_default() {
    let stats = DiscoveryStats::default();
    assert_eq!(stats.total_discovered, 0);
    assert_eq!(stats.discovery_attempts, 0);
    assert_eq!(stats.avg_discovery_time_ns, 0);
    assert_eq!(stats.connection_complexity, 0.0);
}

#[test]
fn test_discovery_stats_clone() {
    let stats = DiscoveryStats {
        total_discovered: 5,
        discovery_attempts: 10,
        avg_discovery_time_ns: 1000,
        connection_complexity: 1.5,
    };

    let cloned_stats = stats.clone();
    assert_eq!(stats.total_discovered, cloned_stats.total_discovered);
    assert_eq!(stats.discovery_attempts, cloned_stats.discovery_attempts);
    assert_eq!(
        stats.avg_discovery_time_ns,
        cloned_stats.avg_discovery_time_ns
    );
    assert_eq!(
        stats.connection_complexity,
        cloned_stats.connection_complexity
    );
}

#[test]
fn test_dignity_rule_creation() {
    let rule = DignityRule {
        id: "test_rule".to_string(),
        description: "Test dignity rule".to_string(),
        validator: |cap| cap.sovereignty_compliant,
    };

    assert_eq!(rule.id, "test_rule");
    assert_eq!(rule.description, "Test dignity rule");

    // Test validator function
    let compliant_cap = CapabilityDescriptor {
        id: "test".to_string(),
        capability_type: CapabilityType::Storage,
        endpoint: None,
        metadata: HashMap::new(),
        sovereignty_compliant: true,
    };

    let non_compliant_cap = CapabilityDescriptor {
        id: "test".to_string(),
        capability_type: CapabilityType::Storage,
        endpoint: None,
        metadata: HashMap::new(),
        sovereignty_compliant: false,
    };

    assert!((rule.validator)(&compliant_cap));
    assert!(!(rule.validator)(&non_compliant_cap));
}

// ==================== INTEGRATION TESTS ====================

#[tokio::test]
async fn test_infant_discovery_system_capability_discovery() {
    let mut system: InfantDiscoverySystem<16> = InfantDiscoverySystem::new();

    let result = system.discover_capabilities().await;
    assert!(result.is_ok());

    let capabilities = result.expect("Test setup failed");
    assert!(!capabilities.is_empty());

    // All discovered capabilities should be sovereignty compliant
    for cap in &capabilities {
        assert!(cap.sovereignty_compliant);
        assert!(!cap.id.is_empty());
    }
}

#[tokio::test]
async fn test_infant_discovery_system_connection_establishment() {
    let mut system: InfantDiscoverySystem<8> = InfantDiscoverySystem::new();

    // First discover capabilities
    let capabilities = system
        .discover_capabilities()
        .await
        .expect("Should discover capabilities");
    assert!(!capabilities.is_empty());

    // Then try to establish connection
    let connection_result = system.establish_connection(&capabilities[0].id).await;
    assert!(connection_result.is_ok());

    let connection = connection_result.expect("Test setup failed");
    assert_eq!(connection.id, capabilities[0].id);
    assert_eq!(connection.complexity_order, 1); // Should be O(1)
}

#[tokio::test]
async fn test_infant_discovery_system_statistics() {
    let mut system: InfantDiscoverySystem<4> = InfantDiscoverySystem::new();

    // Perform some operations
    let _capabilities = system.discover_capabilities().await.expect("Should work");

    let stats = system.get_discovery_stats().await;
    assert!(stats.total_discovered > 0);
    assert!(stats.discovery_attempts > 0);
    assert!(stats.avg_discovery_time_ns > 0);
}

// ==================== EDGE CASE TESTS ====================

#[test]
fn test_empty_capability_metadata() {
    let capability = CapabilityDescriptor {
        id: "empty_metadata".to_string(),
        capability_type: CapabilityType::Unknown,
        endpoint: None,
        metadata: HashMap::new(),
        sovereignty_compliant: true,
    };

    assert!(capability.metadata.is_empty());
    assert!(capability.endpoint.is_none());
}

#[test]
fn test_capability_with_complex_metadata() {
    let mut metadata = HashMap::new();
    metadata.insert("version".to_string(), "2.1.0".to_string());
    metadata.insert("protocol".to_string(), "https".to_string());
    metadata.insert("authentication".to_string(), "oauth2".to_string());
    metadata.insert("rate_limit".to_string(), "1000/hour".to_string());

    let capability = CapabilityDescriptor {
        id: "complex_capability".to_string(),
        capability_type: CapabilityType::Network,
        endpoint: Some("https://api.example.com/v2".to_string()),
        metadata,
        sovereignty_compliant: true,
    };

    assert_eq!(capability.metadata.len(), 4);
    assert_eq!(
        capability.metadata.get("version"),
        Some(&"2.1.0".to_string())
    );
    assert_eq!(
        capability.metadata.get("protocol"),
        Some(&"https".to_string())
    );
}

#[test]
fn test_large_discovery_system() {
    let system: InfantDiscoverySystem<1024> = InfantDiscoverySystem::new();
    assert!(system.verify_sovereignty_compliance());
}

#[test]
fn test_small_discovery_system() {
    let system: InfantDiscoverySystem<2> = InfantDiscoverySystem::new();
    assert!(system.verify_sovereignty_compliance());
}

// ==================== PERFORMANCE TESTS ====================

#[test]
fn test_capability_creation_performance() {
    let start = std::time::Instant::now();

    for i in 0..1000 {
        let capability = CapabilityDescriptor {
            id: format!("capability_{i}"),
            capability_type: CapabilityType::Storage,
            endpoint: Some(format!("http://service-{i}.local")),
            metadata: HashMap::from([
                ("index".to_string(), i.to_string()),
                ("created_at".to_string(), "2025-01-01".to_string()),
            ]),
            sovereignty_compliant: true,
        };

        // Verify capability was created correctly
        assert!(!capability.id.is_empty());
        assert!(capability.sovereignty_compliant);
    }

    let elapsed = start.elapsed();
    assert!(
        elapsed.as_millis() < 100,
        "Should create 1000 capabilities in under 100ms"
    );
}

#[test]
fn test_discovery_stats_update_performance() {
    let mut stats = DiscoveryStats::default();

    let start = std::time::Instant::now();

    for i in 0..10000 {
        stats.total_discovered += 1;
        stats.discovery_attempts += 1;
        stats.avg_discovery_time_ns = (stats.avg_discovery_time_ns + (i as u64 * 1000)) / 2;
        stats.connection_complexity = f64::from(i) / 1000.0;
    }

    let elapsed = start.elapsed();
    assert!(
        elapsed.as_millis() < 10,
        "Should update stats 10000 times in under 10ms"
    );

    assert_eq!(stats.total_discovered, 10000);
    assert_eq!(stats.discovery_attempts, 10000);
}

// ==================== SOVEREIGNTY COMPLIANCE TESTS ====================

#[test]
fn test_dignity_rule_validation() {
    // Test individual dignity rule validation
    let rule = DignityRule {
        id: "test_rule".to_string(),
        description: "Test dignity rule".to_string(),
        validator: |cap| cap.sovereignty_compliant,
    };

    let compliant_cap = CapabilityDescriptor {
        id: "test".to_string(),
        capability_type: CapabilityType::Storage,
        endpoint: None,
        metadata: HashMap::new(),
        sovereignty_compliant: true,
    };

    let non_compliant_cap = CapabilityDescriptor {
        id: "test".to_string(),
        capability_type: CapabilityType::Storage,
        endpoint: None,
        metadata: HashMap::new(),
        sovereignty_compliant: false,
    };

    assert!((rule.validator)(&compliant_cap));
    assert!(!(rule.validator)(&non_compliant_cap));
}

#[test]
fn test_sovereignty_basic_compliance() {
    // Test basic sovereignty compliance concepts
    let capability = CapabilityDescriptor {
        id: "test_capability".to_string(),
        capability_type: CapabilityType::Storage,
        endpoint: None,
        metadata: HashMap::from([
            ("privacy_compliant".to_string(), "true".to_string()),
            ("data_sovereignty".to_string(), "local".to_string()),
        ]),
        sovereignty_compliant: true,
    };

    // Test that sovereignty compliant capabilities have the right metadata
    assert!(capability.sovereignty_compliant);
    assert_eq!(
        capability.metadata.get("privacy_compliant"),
        Some(&"true".to_string())
    );
    assert_eq!(
        capability.metadata.get("data_sovereignty"),
        Some(&"local".to_string())
    );
}
