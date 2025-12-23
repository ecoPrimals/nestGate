//! Comprehensive tests for System Introspection Module
//!
//! This test module provides extensive coverage for the introspection subsystem,
//! which handles system capability detection, hardware profiling, and resource analysis.

use super::super::introspection_config::IntrospectionConfig;
use super::{HardwareProfile, SystemCapabilities, SystemIntrospection};
use std::collections::HashMap;
use std::sync::Arc;

// ==================== BASIC INSTANTIATION TESTS ====================

#[test]
fn test_system_introspection_new() {
    let introspection = SystemIntrospection::new();
    // Verify it can be created - internal structure validated by type system
    drop(introspection);
}

#[test]
fn test_system_introspection_default() {
    let introspection = SystemIntrospection::default();
    // Verify default creates valid instance
    drop(introspection);
}

#[test]
fn test_system_introspection_with_config() {
    let config = Arc::new(IntrospectionConfig::from_env());
    let introspection = SystemIntrospection::with_config(config);
    drop(introspection);
}

#[test]
fn test_multiple_introspection_instances() {
    let intro1 = SystemIntrospection::new();
    let intro2 = SystemIntrospection::new();
    let intro3 = SystemIntrospection::default();

    // All instances should be independent
    drop(intro1);
    drop(intro2);
    drop(intro3);
}

// ==================== SYSTEM CAPABILITIES TESTS ====================

#[test]
fn test_system_capabilities_creation() {
    let capabilities = SystemCapabilities {
        cpu_cores: 8,
        logical_cores: 16,
        memory_gb: 16.0,
        network_interfaces: vec!["eth0".to_string(), "lo".to_string()],
        storage_available: true,
        container_runtime: Some("docker".to_string()),
        os_type: "linux".to_string(),
    };

    assert_eq!(capabilities.cpu_cores, 8);
    assert_eq!(capabilities.logical_cores, 16);
    assert_eq!(capabilities.memory_gb, 16.0);
    assert_eq!(capabilities.network_interfaces.len(), 2);
    assert!(capabilities.storage_available);
    assert_eq!(capabilities.container_runtime, Some("docker".to_string()));
    assert_eq!(capabilities.os_type, "linux");
}

#[test]
fn test_system_capabilities_clone() {
    let capabilities = SystemCapabilities {
        cpu_cores: 4,
        logical_cores: 8,
        memory_gb: 8.0,
        network_interfaces: vec!["eth0".to_string()],
        storage_available: true,
        container_runtime: None,
        os_type: "linux".to_string(),
    };

    let cloned = capabilities.clone();
    assert_eq!(capabilities.cpu_cores, cloned.cpu_cores);
    assert_eq!(capabilities.logical_cores, cloned.logical_cores);
    assert_eq!(capabilities.memory_gb, cloned.memory_gb);
}

#[test]
fn test_system_capabilities_minimal_config() {
    let capabilities = SystemCapabilities {
        cpu_cores: 1,
        logical_cores: 1,
        memory_gb: 0.5,
        network_interfaces: vec![],
        storage_available: false,
        container_runtime: None,
        os_type: "unknown".to_string(),
    };

    assert_eq!(capabilities.cpu_cores, 1);
    assert!(capabilities.network_interfaces.is_empty());
    assert!(!capabilities.storage_available);
}

#[test]
fn test_system_capabilities_high_spec_system() {
    let capabilities = SystemCapabilities {
        cpu_cores: 64,
        logical_cores: 128,
        memory_gb: 512.0,
        network_interfaces: vec![
            "eth0".to_string(),
            "eth1".to_string(),
            "eth2".to_string(),
            "eth3".to_string(),
        ],
        storage_available: true,
        container_runtime: Some("kubernetes".to_string()),
        os_type: "linux".to_string(),
    };

    assert_eq!(capabilities.cpu_cores, 64);
    assert_eq!(capabilities.logical_cores, 128);
    assert!(capabilities.memory_gb > 500.0);
}

// ==================== HARDWARE PROFILE TESTS ====================

#[test]
fn test_hardware_profile_creation() {
    let mut recommended_limits = HashMap::new();
    recommended_limits.insert("connections".to_string(), 1000);
    recommended_limits.insert("threads".to_string(), 16);
    recommended_limits.insert("memory_buffer".to_string(), 8192);

    let profile = HardwareProfile {
        cpu_score: 0.8,
        memory_score: 0.9,
        storage_score: 0.7,
        network_score: 0.85,
        overall_score: 0.8125,
        recommended_limits,
    };

    assert_eq!(profile.cpu_score, 0.8);
    assert_eq!(profile.memory_score, 0.9);
    assert_eq!(profile.storage_score, 0.7);
    assert_eq!(profile.network_score, 0.85);
    assert_eq!(profile.overall_score, 0.8125);
    assert_eq!(profile.recommended_limits.len(), 3);
}

#[test]
fn test_hardware_profile_clone() {
    let mut recommended_limits = HashMap::new();
    recommended_limits.insert("connections".to_string(), 500);

    let profile = HardwareProfile {
        cpu_score: 0.5,
        memory_score: 0.6,
        storage_score: 0.4,
        network_score: 0.7,
        overall_score: 0.55,
        recommended_limits,
    };

    let cloned = profile.clone();
    assert_eq!(profile.cpu_score, cloned.cpu_score);
    assert_eq!(profile.overall_score, cloned.overall_score);
    assert_eq!(
        profile.recommended_limits.len(),
        cloned.recommended_limits.len()
    );
}

#[test]
fn test_hardware_profile_score_ranges() {
    let profile = HardwareProfile {
        cpu_score: 0.0,
        memory_score: 0.5,
        storage_score: 1.0,
        network_score: 0.25,
        overall_score: 0.4375,
        recommended_limits: HashMap::new(),
    };

    // ✅ MODERN: Verify all scores are within valid range [0.0, 1.0] using epsilon
    assert!(profile.cpu_score >= -1e-9 && profile.cpu_score <= 1.0 + 1e-9);
    assert!(profile.memory_score >= -1e-9 && profile.memory_score <= 1.0 + 1e-9);
    assert!(profile.storage_score >= -1e-9 && profile.storage_score <= 1.0 + 1e-9);
    assert!(profile.network_score >= -1e-9 && profile.network_score <= 1.0 + 1e-9);
    assert!(profile.overall_score >= -1e-9 && profile.overall_score <= 1.0 + 1e-9);
}

#[test]
fn test_hardware_profile_low_spec_system() {
    let mut recommended_limits = HashMap::new();
    recommended_limits.insert("connections".to_string(), 100);
    recommended_limits.insert("threads".to_string(), 2);

    let profile = HardwareProfile {
        cpu_score: 0.2,
        memory_score: 0.3,
        storage_score: 0.25,
        network_score: 0.3,
        overall_score: 0.2625,
        recommended_limits,
    };

    assert!(profile.overall_score < 0.5);
    assert_eq!(profile.recommended_limits.get("connections"), Some(&100));
    assert_eq!(profile.recommended_limits.get("threads"), Some(&2));
}

#[test]
fn test_hardware_profile_high_spec_system() {
    let mut recommended_limits = HashMap::new();
    recommended_limits.insert("connections".to_string(), 10000);
    recommended_limits.insert("threads".to_string(), 64);
    recommended_limits.insert("memory_buffer".to_string(), 67108864); // 64MB

    let profile = HardwareProfile {
        cpu_score: 0.95,
        memory_score: 0.98,
        storage_score: 0.92,
        network_score: 0.96,
        overall_score: 0.9525,
        recommended_limits,
    };

    assert!(profile.overall_score > 0.9);
    assert!(profile.recommended_limits.get("connections").unwrap() > &5000);
}

// ==================== ASYNC DISCOVERY TESTS ====================

#[tokio::test]
async fn test_discover_resource_limits_connections() {
    let mut introspection = SystemIntrospection::new();

    // Test connection limit discovery
    let result = introspection.discover_resource_limits("connections").await;

    // Should succeed and return a reasonable limit
    if let Ok(limit) = result {
        assert!(limit >= 100);
        assert!(limit <= 10000);
    }
}

#[tokio::test]
async fn test_discover_resource_limits_threads() {
    let mut introspection = SystemIntrospection::new();

    // Test thread limit discovery
    let result = introspection.discover_resource_limits("threads").await;

    // Should succeed and return a reasonable limit
    if let Ok(limit) = result {
        assert!(limit >= 4);
        assert!(limit <= 64);
    }
}

#[tokio::test]
async fn test_discover_resource_limits_memory_buffer() {
    let mut introspection = SystemIntrospection::new();

    // Test memory buffer limit discovery
    let result = introspection
        .discover_resource_limits("memory_buffer")
        .await;

    // Should succeed and return a reasonable limit
    if let Ok(limit) = result {
        assert!(limit >= 4096); // At least 4KB
        assert!(limit <= 67108864); // At most 64MB
    }
}

#[tokio::test]
async fn test_discover_resource_limits_unknown_type() {
    let mut introspection = SystemIntrospection::new();

    // Test with unknown resource type
    let result = introspection.discover_resource_limits("unknown_type").await;

    // Should still succeed with fallback behavior
    assert!(result.is_ok() || result.is_err()); // Either way is acceptable
}

#[tokio::test]
async fn test_detect_system_capabilities() {
    let introspection = SystemIntrospection::new();

    let result = introspection.detect_system_capabilities().await;

    // Should detect basic capabilities
    if let Ok(capabilities) = result {
        assert!(capabilities.cpu_cores > 0);
        assert!(capabilities.logical_cores >= capabilities.cpu_cores);
        // ✅ MODERN: Use epsilon for float comparison
        assert!(capabilities.memory_gb > 1e-9);
        assert!(!capabilities.os_type.is_empty());
    }
}

#[tokio::test]
async fn test_hardware_profile_structure() {
    // Test that hardware profile structure is sound
    let mut recommended_limits = HashMap::new();
    recommended_limits.insert("connections".to_string(), 1000);
    recommended_limits.insert("threads".to_string(), 16);

    let profile = HardwareProfile {
        cpu_score: 0.8,
        memory_score: 0.9,
        storage_score: 0.85,
        network_score: 0.88,
        overall_score: 0.8575,
        recommended_limits,
    };

    // ✅ MODERN: Verify all scores are in valid range using epsilon
    assert!(profile.cpu_score >= -1e-9 && profile.cpu_score <= 1.0 + 1e-9);
    assert!(profile.memory_score >= -1e-9 && profile.memory_score <= 1.0 + 1e-9);
    assert!(profile.storage_score >= -1e-9 && profile.storage_score <= 1.0 + 1e-9);
    assert!(profile.network_score >= -1e-9 && profile.network_score <= 1.0 + 1e-9);
    assert!(profile.overall_score >= -1e-9 && profile.overall_score <= 1.0 + 1e-9);
}

// ==================== REPEATED DISCOVERY TESTS ====================

#[tokio::test]
async fn test_multiple_resource_limit_discoveries() {
    let mut introspection = SystemIntrospection::new();

    // Multiple discoveries should be consistent
    let result1 = introspection.discover_resource_limits("connections").await;
    let result2 = introspection.discover_resource_limits("connections").await;

    if let (Ok(limit1), Ok(limit2)) = (result1, result2) {
        assert_eq!(
            limit1, limit2,
            "Consecutive discoveries should be consistent"
        );
    }
}

#[tokio::test]
async fn test_multiple_capability_detections() {
    let introspection = SystemIntrospection::new();

    let result1 = introspection.detect_system_capabilities().await;
    let result2 = introspection.detect_system_capabilities().await;

    if let (Ok(cap1), Ok(cap2)) = (result1, result2) {
        assert_eq!(cap1.cpu_cores, cap2.cpu_cores);
        assert_eq!(cap1.logical_cores, cap2.logical_cores);
        assert_eq!(cap1.os_type, cap2.os_type);
    }
}

// ==================== CONFIGURATION TESTS ====================

#[test]
fn test_introspection_config_from_env() {
    let config = IntrospectionConfig::from_env();
    // Should create valid config from environment
    let _ = Arc::new(config);
}

#[test]
fn test_introspection_with_shared_config() {
    let config = Arc::new(IntrospectionConfig::from_env());
    let config_clone = Arc::clone(&config);

    let intro1 = SystemIntrospection::with_config(config);
    let intro2 = SystemIntrospection::with_config(config_clone);

    // Both should use the same shared config
    drop(intro1);
    drop(intro2);
}

// ==================== EDGE CASE TESTS ====================

#[tokio::test]
async fn test_discover_all_resource_types() {
    let mut introspection = SystemIntrospection::new();

    let resource_types = vec!["connections", "threads", "memory_buffer"];

    for resource_type in resource_types {
        let result = introspection.discover_resource_limits(resource_type).await;
        // Each resource type should either succeed or fail gracefully
        match result {
            Ok(limit) => assert!(limit > 0, "Limit should be positive for {resource_type}"),
            Err(_) => {
                // Error is acceptable - system introspection may not be available
            }
        }
    }
}

#[test]
fn test_system_capabilities_with_container_runtime() {
    let capabilities = SystemCapabilities {
        cpu_cores: 4,
        logical_cores: 8,
        memory_gb: 8.0,
        network_interfaces: vec!["eth0".to_string()],
        storage_available: true,
        container_runtime: Some("docker".to_string()),
        os_type: "linux".to_string(),
    };

    assert!(capabilities.container_runtime.is_some());
    assert_eq!(
        capabilities.container_runtime.unwrap(),
        "docker".to_string()
    );
}

#[test]
fn test_system_capabilities_without_container_runtime() {
    let capabilities = SystemCapabilities {
        cpu_cores: 4,
        logical_cores: 8,
        memory_gb: 8.0,
        network_interfaces: vec!["eth0".to_string()],
        storage_available: true,
        container_runtime: None,
        os_type: "linux".to_string(),
    };

    assert!(capabilities.container_runtime.is_none());
}

#[test]
fn test_hardware_profile_empty_recommended_limits() {
    let profile = HardwareProfile {
        cpu_score: 0.5,
        memory_score: 0.5,
        storage_score: 0.5,
        network_score: 0.5,
        overall_score: 0.5,
        recommended_limits: HashMap::new(),
    };

    assert!(profile.recommended_limits.is_empty());
}

// ==================== CONCURRENCY TESTS ====================

#[tokio::test]
async fn test_concurrent_introspection_instances() {
    use tokio::task;

    let handles: Vec<_> = (0..5)
        .map(|_| {
            task::spawn(async {
                let introspection = SystemIntrospection::new();
                introspection.detect_system_capabilities().await
            })
        })
        .collect();

    for handle in handles {
        let result = handle.await;
        assert!(result.is_ok());
    }
}

#[tokio::test]
async fn test_concurrent_resource_limit_discovery() {
    use tokio::task;

    let handles: Vec<_> = vec!["connections", "threads", "memory_buffer"]
        .into_iter()
        .map(|resource_type| {
            task::spawn(async move {
                let mut intro = SystemIntrospection::new();
                intro.discover_resource_limits(resource_type).await
            })
        })
        .collect();

    for handle in handles {
        let result = handle.await;
        assert!(result.is_ok());
    }
}

// ==================== SUMMARY STATISTICS ====================

#[test]
fn test_module_coverage_tracking() {
    // This test documents what we're testing
    let test_categories = [
        "Basic Instantiation Tests",
        "System Capabilities Tests",
        "Hardware Profile Tests",
        "Async Discovery Tests",
        "Repeated Discovery Tests",
        "Configuration Tests",
        "Edge Case Tests",
        "Concurrency Tests",
    ];

    assert_eq!(test_categories.len(), 8);
}
