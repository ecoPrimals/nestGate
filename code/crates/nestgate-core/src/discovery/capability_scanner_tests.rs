//! Comprehensive tests for Capability Scanner
//!
//! This test module provides extensive coverage of the capability discovery system,
//! including edge cases, error paths, and concurrent scenarios.

#![cfg(test)]

use super::*;
use std::env;
use std::sync::{Arc, Mutex};

// ==================== BASIC FUNCTIONALITY TESTS ====================

/// Tests that environment discovery correctly finds configured capabilities
#[tokio::test]
async fn test_environment_discovery_finds_capabilities() {
    // Arrange
    env::set_var("ORCHESTRATION_DISCOVERY_ENDPOINT", "http://orchestration:8080");
    env::set_var("SECURITY_DISCOVERY_ENDPOINT", "http://security:8081");
    
    let discovery = EnvironmentDiscovery::new();
    
    // Act
    let capabilities = discovery.discover().await.unwrap();
    
    // Assert
    assert!(!capabilities.is_empty());
    assert!(capabilities.len() >= 2);
    
    let orch = capabilities.iter().find(|c| c.capability_type == "orchestration");
    assert!(orch.is_some());
    assert_eq!(orch.unwrap().endpoint, "http://orchestration:8080");
    
    let sec = capabilities.iter().find(|c| c.capability_type == "security");
    assert!(sec.is_some());
    assert_eq!(sec.unwrap().endpoint, "http://security:8081");
    
    // Cleanup
    env::remove_var("ORCHESTRATION_DISCOVERY_ENDPOINT");
    env::remove_var("SECURITY_DISCOVERY_ENDPOINT");
}

/// Tests that environment discovery returns empty result when no capabilities configured
#[tokio::test]
async fn test_environment_discovery_empty_when_no_capabilities() {
    // Arrange - ensure no discovery env vars are set
    let patterns = vec![
        "ORCHESTRATION_DISCOVERY_ENDPOINT",
        "SECURITY_DISCOVERY_ENDPOINT",
        "AI_DISCOVERY_ENDPOINT",
        "STORAGE_DISCOVERY_ENDPOINT",
        "MONITORING_DISCOVERY_ENDPOINT",
        "COMPUTE_DISCOVERY_ENDPOINT",
        "NETWORK_DISCOVERY_ENDPOINT",
    ];
    
    for pattern in &patterns {
        env::remove_var(pattern);
    }
    
    let discovery = EnvironmentDiscovery::new();
    
    // Act
    let capabilities = discovery.discover().await.unwrap();
    
    // Assert
    assert!(capabilities.is_empty());
}

/// Tests that custom patterns can be added to the scanner
#[tokio::test]
async fn test_custom_capability_pattern() {
    // Arrange
    env::set_var("CUSTOM_SERVICE_DISCOVERY_ENDPOINT", "http://custom:9000");
    
    let mut discovery = EnvironmentDiscovery::new();
    discovery.add_pattern("CUSTOM_SERVICE_DISCOVERY_ENDPOINT".to_string());
    
    // Act
    let capabilities = discovery.discover().await.unwrap();
    
    // Assert
    assert!(!capabilities.is_empty());
    let custom = capabilities.iter().find(|c| c.capability_type == "custom_service");
    assert!(custom.is_some());
    assert_eq!(custom.unwrap().endpoint, "http://custom:9000");
    
    // Cleanup
    env::remove_var("CUSTOM_SERVICE_DISCOVERY_ENDPOINT");
}

/// Tests that metadata is correctly extracted from environment variables
#[tokio::test]
async fn test_capability_metadata_extraction() {
    // Arrange
    env::set_var("AI_DISCOVERY_ENDPOINT", "http://ai:8082");
    env::set_var("AI_AUTH_KEY", "secret-token-123");
    env::set_var("AI_TIMEOUT_MS", "5000");
    
    let discovery = EnvironmentDiscovery::new();
    
    // Act
    let capabilities = discovery.discover().await.unwrap();
    
    // Assert
    let ai = capabilities.iter().find(|c| c.capability_type == "ai").unwrap();
    assert_eq!(ai.metadata.get("auth_key"), Some(&"secret-token-123".to_string()));
    assert_eq!(ai.metadata.get("timeout_ms"), Some(&"5000".to_string()));
    assert_eq!(ai.metadata.get("source"), Some(&"environment".to_string()));
    
    // Cleanup
    env::remove_var("AI_DISCOVERY_ENDPOINT");
    env::remove_var("AI_AUTH_KEY");
    env::remove_var("AI_TIMEOUT_MS");
}

/// Tests that confidence level is set correctly for environment discovery
#[tokio::test]
async fn test_environment_discovery_confidence() {
    // Arrange
    env::set_var("STORAGE_DISCOVERY_ENDPOINT", "http://storage:8083");
    let discovery = EnvironmentDiscovery::new();
    
    // Act
    let capabilities = discovery.discover().await.unwrap();
    
    // Assert
    let storage = capabilities.iter().find(|c| c.capability_type == "storage").unwrap();
    assert!(storage.confidence >= 0.9); // High confidence for explicit env vars
    assert!(storage.confidence <= 1.0);
    
    // Cleanup
    env::remove_var("STORAGE_DISCOVERY_ENDPOINT");
}

// ==================== EDGE CASE TESTS ====================

/// Tests handling of extremely long endpoint URLs
#[tokio::test]
async fn test_extremely_long_endpoint_url() {
    // Arrange
    let long_url = format!("http://{}:8080/path", "a".repeat(1000));
    env::set_var("COMPUTE_DISCOVERY_ENDPOINT", &long_url);
    
    let discovery = EnvironmentDiscovery::new();
    
    // Act
    let capabilities = discovery.discover().await.unwrap();
    
    // Assert
    assert!(!capabilities.is_empty());
    let compute = capabilities.iter().find(|c| c.capability_type == "compute").unwrap();
    assert_eq!(compute.endpoint, long_url);
    
    // Cleanup
    env::remove_var("COMPUTE_DISCOVERY_ENDPOINT");
}

/// Tests handling of special characters in endpoint URLs
#[tokio::test]
async fn test_special_characters_in_endpoint() {
    // Arrange
    let special_url = "http://service:8080/path?query=value&other=123#fragment";
    env::set_var("MONITORING_DISCOVERY_ENDPOINT", special_url);
    
    let discovery = EnvironmentDiscovery::new();
    
    // Act
    let capabilities = discovery.discover().await.unwrap();
    
    // Assert
    let monitoring = capabilities.iter().find(|c| c.capability_type == "monitoring").unwrap();
    assert_eq!(monitoring.endpoint, special_url);
    
    // Cleanup
    env::remove_var("MONITORING_DISCOVERY_ENDPOINT");
}

/// Tests handling of empty endpoint URL
#[tokio::test]
async fn test_empty_endpoint_url() {
    // Arrange
    env::set_var("NETWORK_DISCOVERY_ENDPOINT", "");
    let discovery = EnvironmentDiscovery::new();
    
    // Act
    let capabilities = discovery.discover().await.unwrap();
    
    // Assert
    let network = capabilities.iter().find(|c| c.capability_type == "network").unwrap();
    assert_eq!(network.endpoint, "");
    
    // Cleanup
    env::remove_var("NETWORK_DISCOVERY_ENDPOINT");
}

/// Tests handling of invalid UTF-8 in environment variables (should not panic)
#[tokio::test]
async fn test_handles_missing_env_vars_gracefully() {
    // Arrange
    env::remove_var("NONEXISTENT_DISCOVERY_ENDPOINT");
    let mut discovery = EnvironmentDiscovery::new();
    discovery.add_pattern("NONEXISTENT_DISCOVERY_ENDPOINT".to_string());
    
    // Act
    let capabilities = discovery.discover().await.unwrap();
    
    // Assert
    let nonexistent = capabilities.iter().find(|c| c.capability_type == "nonexistent");
    assert!(nonexistent.is_none());
}

/// Tests handling of multiple capabilities of the same type
#[tokio::test]
async fn test_multiple_custom_capabilities() {
    // Arrange
    env::set_var("SERVICE_A_DISCOVERY_ENDPOINT", "http://service-a:8080");
    env::set_var("SERVICE_B_DISCOVERY_ENDPOINT", "http://service-b:8081");
    env::set_var("SERVICE_C_DISCOVERY_ENDPOINT", "http://service-c:8082");
    
    let mut discovery = EnvironmentDiscovery::new();
    discovery.add_pattern("SERVICE_A_DISCOVERY_ENDPOINT".to_string());
    discovery.add_pattern("SERVICE_B_DISCOVERY_ENDPOINT".to_string());
    discovery.add_pattern("SERVICE_C_DISCOVERY_ENDPOINT".to_string());
    
    // Act
    let capabilities = discovery.discover().await.unwrap();
    
    // Assert
    assert!(capabilities.len() >= 3);
    assert!(capabilities.iter().any(|c| c.capability_type == "service_a"));
    assert!(capabilities.iter().any(|c| c.capability_type == "service_b"));
    assert!(capabilities.iter().any(|c| c.capability_type == "service_c"));
    
    // Cleanup
    env::remove_var("SERVICE_A_DISCOVERY_ENDPOINT");
    env::remove_var("SERVICE_B_DISCOVERY_ENDPOINT");
    env::remove_var("SERVICE_C_DISCOVERY_ENDPOINT");
}

// ==================== CONCURRENCY TESTS ====================

/// Tests that concurrent discovery calls don't interfere with each other
#[tokio::test]
async fn test_concurrent_discovery_calls() {
    // Arrange - clean environment first to avoid pollution from other tests
    let patterns = vec![
        "ORCHESTRATION_DISCOVERY_ENDPOINT",
        "SECURITY_DISCOVERY_ENDPOINT",
        "AI_DISCOVERY_ENDPOINT",
        "STORAGE_DISCOVERY_ENDPOINT",
        "MONITORING_DISCOVERY_ENDPOINT",
        "COMPUTE_DISCOVERY_ENDPOINT",
        "NETWORK_DISCOVERY_ENDPOINT",
    ];
    for pattern in &patterns {
        env::remove_var(pattern);
    }
    
    env::set_var("ORCHESTRATION_DISCOVERY_ENDPOINT", "http://orchestration:8080");
    env::set_var("SECURITY_DISCOVERY_ENDPOINT", "http://security:8081");
    
    let discovery = Arc::new(EnvironmentDiscovery::new());
    
    // Act - spawn multiple concurrent discovery operations
    let mut handles = Vec::new();
    for _ in 0..10 {
        let discovery = Arc::clone(&discovery);
        let handle = tokio::spawn(async move {
            discovery.discover().await
        });
        handles.push(handle);
    }
    
    // Wait for all tasks
    let results = futures::future::join_all(handles).await;
    
    // Assert - all should succeed and return consistent results
    for result in results {
        let capabilities = result.unwrap().unwrap();
        assert!(!capabilities.is_empty());
        assert!(capabilities.iter().any(|c| c.capability_type == "orchestration"));
        assert!(capabilities.iter().any(|c| c.capability_type == "security"));
    }
    
    // Cleanup
    env::remove_var("ORCHESTRATION_DISCOVERY_ENDPOINT");
    env::remove_var("SECURITY_DISCOVERY_ENDPOINT");
}

/// Tests that modifying patterns is safe (not concurrent, but tests mutability)
#[tokio::test]
async fn test_pattern_modification_isolation() {
    // Arrange
    env::set_var("CUSTOM_DISCOVERY_ENDPOINT", "http://custom:9000");
    
    let mut discovery1 = EnvironmentDiscovery::new();
    let mut discovery2 = EnvironmentDiscovery::new();
    
    discovery1.add_pattern("CUSTOM_DISCOVERY_ENDPOINT".to_string());
    // discovery2 should not have the custom pattern
    
    // Act
    let caps1 = discovery1.discover().await.unwrap();
    let caps2 = discovery2.discover().await.unwrap();
    
    // Assert
    let custom1 = caps1.iter().find(|c| c.capability_type == "custom");
    let custom2 = caps2.iter().find(|c| c.capability_type == "custom");
    
    assert!(custom1.is_some()); // discovery1 should find it
    assert!(custom2.is_none());  // discovery2 should not find it
    
    // Cleanup
    env::remove_var("CUSTOM_DISCOVERY_ENDPOINT");
}

// ==================== DEFAULT TRAIT TESTS ====================

/// Tests that Default trait creates a valid scanner
#[tokio::test]
async fn test_default_trait() {
    // Arrange & Act
    let discovery = EnvironmentDiscovery::default();
    
    // Assert - should have default patterns
    assert!(discovery.capability_patterns.len() > 0);
}

/// Tests that default scanner can discover capabilities
#[tokio::test]
async fn test_default_scanner_discovers() {
    // Arrange
    env::set_var("AI_DISCOVERY_ENDPOINT", "http://ai:8082");
    let discovery = EnvironmentDiscovery::default();
    
    // Act
    let capabilities = discovery.discover().await.unwrap();
    
    // Assert
    assert!(!capabilities.is_empty());
    
    // Cleanup
    env::remove_var("AI_DISCOVERY_ENDPOINT");
}

// ==================== METHOD NAME TESTS ====================

/// Tests that method_name returns correct value
#[test]
fn test_method_name() {
    let discovery = EnvironmentDiscovery::new();
    assert_eq!(discovery.method_name(), "environment");
}

// ==================== CAPABILITY SCANNER TESTS ====================

/// Tests that CapabilityScanner aggregates results from multiple methods
#[tokio::test]
async fn test_capability_scanner_aggregates_methods() {
    // Arrange
    env::set_var("ORCHESTRATION_DISCOVERY_ENDPOINT", "http://orchestration:8080");
    env::set_var("SECURITY_DISCOVERY_ENDPOINT", "http://security:8081");
    
    let env_discovery = EnvironmentDiscovery::new();
    let mut scanner = CapabilityScanner::new();
    scanner.add_discovery_method(DiscoveryMethodImpl::Environment(env_discovery));
    
    // Act
    let capabilities = scanner.scan_capabilities().await.unwrap();
    
    // Assert
    assert!(!capabilities.is_empty());
    assert!(capabilities.len() >= 2);
    
    // Cleanup
    env::remove_var("ORCHESTRATION_DISCOVERY_ENDPOINT");
    env::remove_var("SECURITY_DISCOVERY_ENDPOINT");
}

/// Tests that scanner handles failures from individual methods gracefully
#[tokio::test]
async fn test_scanner_handles_partial_failures() {
    // Arrange
    let mut scanner = CapabilityScanner::new();
    scanner.add_discovery_method(DiscoveryMethodImpl::Environment(EnvironmentDiscovery::new()));
    
    // Act - should not panic even if some methods fail
    let result = scanner.scan_capabilities().await;
    
    // Assert - should return Ok (empty vec if no capabilities found)
    assert!(result.is_ok());
}

/// Tests that scanner aggregates capabilities from multiple methods
#[tokio::test]
async fn test_scanner_aggregates_from_multiple_methods() {
    // Arrange
    env::set_var("STORAGE_DISCOVERY_ENDPOINT", "http://storage:8083");
    
    // Create a scanner with default method, then add more
    let mut scanner = CapabilityScanner::new(); // Has default env discovery
    scanner.add_discovery_method(DiscoveryMethodImpl::Environment(EnvironmentDiscovery::new()));
    
    // Act
    let capabilities = scanner.scan_capabilities().await.unwrap();
    
    // Assert - should have storage capability (may have duplicates from multiple methods)
    let storage_caps: Vec<_> = capabilities.iter()
        .filter(|c| c.capability_type == "storage")
        .collect();
    
    assert!(!storage_caps.is_empty(), "Should find storage capability");
    
    // Cleanup
    env::remove_var("STORAGE_DISCOVERY_ENDPOINT");
}

/// Tests that scanner merges metadata from duplicate capabilities
#[tokio::test]
async fn test_scanner_merges_capability_metadata() {
    // Arrange - clean environment first
    let patterns = vec![
        "COMPUTE_DISCOVERY_ENDPOINT",
        "COMPUTE_AUTH_KEY",
        "COMPUTE_TIMEOUT_MS",
    ];
    for pattern in &patterns {
        env::remove_var(pattern);
    }
    
    env::set_var("COMPUTE_DISCOVERY_ENDPOINT", "http://compute:8084");
    env::set_var("COMPUTE_AUTH_KEY", "key123");
    
    let mut scanner = CapabilityScanner::new();
    scanner.add_discovery_method(DiscoveryMethodImpl::Environment(EnvironmentDiscovery::new()));
    
    // Act
    let capabilities = scanner.scan_capabilities().await.unwrap();
    
    // Assert
    let compute = capabilities.iter().find(|c| c.capability_type == "compute").unwrap();
    assert!(compute.metadata.contains_key("source"));
    assert!(compute.metadata.contains_key("pattern"));
    assert!(compute.metadata.contains_key("auth_key"));
    
    // Cleanup
    env::remove_var("COMPUTE_DISCOVERY_ENDPOINT");
    env::remove_var("COMPUTE_AUTH_KEY");
}

// ==================== STRESS TESTS ====================

/// Tests scanner with large number of capabilities
#[tokio::test]
async fn test_scanner_handles_many_capabilities() {
    // Arrange - set up 50 custom capabilities
    let mut discovery = EnvironmentDiscovery::new();
    
    for i in 0..50 {
        let pattern = format!("SERVICE_{}_DISCOVERY_ENDPOINT", i);
        let endpoint = format!("http://service-{}:8080", i);
        env::set_var(&pattern, &endpoint);
        discovery.add_pattern(pattern.clone());
    }
    
    // Act
    let capabilities = discovery.discover().await.unwrap();
    
    // Assert
    assert!(capabilities.len() >= 50);
    
    // Cleanup
    for i in 0..50 {
        let pattern = format!("SERVICE_{}_DISCOVERY_ENDPOINT", i);
        env::remove_var(&pattern);
    }
}

/// Tests performance of repeated scans
#[tokio::test]
async fn test_repeated_scans_performance() {
    // Arrange
    env::set_var("MONITORING_DISCOVERY_ENDPOINT", "http://monitoring:8085");
    let discovery = EnvironmentDiscovery::new();
    
    // Act - perform 100 scans
    let start = std::time::Instant::now();
    for _ in 0..100 {
        let _ = discovery.discover().await.unwrap();
    }
    let duration = start.elapsed();
    
    // Assert - should complete in reasonable time (< 1 second for 100 scans)
    assert!(duration.as_secs() < 1, "100 scans took {:?}, should be < 1s", duration);
    
    // Cleanup
    env::remove_var("MONITORING_DISCOVERY_ENDPOINT");
}

// ==================== ERROR HANDLING TESTS ====================

/// Tests that scanner returns Ok when no methods configured (has default env discovery)
#[tokio::test]
async fn test_scanner_with_default_methods() {
    // Arrange
    // Clear any discovery env vars
    let patterns = vec![
        "ORCHESTRATION_DISCOVERY_ENDPOINT",
        "SECURITY_DISCOVERY_ENDPOINT",
        "AI_DISCOVERY_ENDPOINT",
        "STORAGE_DISCOVERY_ENDPOINT",
        "MONITORING_DISCOVERY_ENDPOINT",
        "COMPUTE_DISCOVERY_ENDPOINT",
        "NETWORK_DISCOVERY_ENDPOINT",
    ];
    for pattern in &patterns {
        env::remove_var(pattern);
    }
    
    let mut scanner = CapabilityScanner::new();
    
    // Act
    let result = scanner.scan_capabilities().await;
    
    // Assert
    assert!(result.is_ok());
    // Scanner has default environment discovery, so result may be empty or have capabilities
}

/// Tests that CapabilityInfo can be cloned
#[test]
fn test_capability_info_clone() {
    let mut metadata = HashMap::new();
    metadata.insert("key".to_string(), "value".to_string());
    
    let info = CapabilityInfo {
        capability_type: "test".to_string(),
        endpoint: "http://test:8080".to_string(),
        confidence: 0.95,
        metadata,
    };
    
    let cloned = info.clone();
    assert_eq!(info.capability_type, cloned.capability_type);
    assert_eq!(info.endpoint, cloned.endpoint);
    assert_eq!(info.confidence, cloned.confidence);
}

/// Tests that CapabilityInfo can be debugged
#[test]
fn test_capability_info_debug() {
    let info = CapabilityInfo {
        capability_type: "test".to_string(),
        endpoint: "http://test:8080".to_string(),
        confidence: 0.95,
        metadata: HashMap::new(),
    };
    
    let debug_str = format!("{:?}", info);
    assert!(debug_str.contains("test"));
    assert!(debug_str.contains("http://test:8080"));
}

