//! Modern Concurrent-Safe Tests for Capability Scanner
//!
//! These tests use dependency injection with immutable configuration instead of
//! environment variable manipulation. This eliminates race conditions and makes
//! tests truly parallel-safe.
//!
//! # Key Improvements
//!
//! 1. **No `env::set_var()`** - No global state pollution
//! 2. **No `env::remove_var()`** - No cleanup needed
//! 3. **Truly parallel** - Tests can run concurrently without interference
//! 4. **More realistic** - Tests production code paths (config injection)

use super::super::capability_scanner_config::EnvironmentDiscoveryConfig;
use super::*;
use std::sync::Arc;

// ==================== BASIC FUNCTIONALITY TESTS ====================

/// Tests that discovery finds capabilities from injected config
#[tokio::test]
async fn test_discovery_with_injected_config() {
    // Arrange - inject config directly, no env vars!
    let mut config = EnvironmentDiscoveryConfig::new();
    config.set_endpoint("orchestration", "http://orchestration:8080");
    config.set_endpoint("security", "http://security:8081");

    let discovery = EnvironmentDiscovery::with_config(Arc::new(config));

    // Act
    let capabilities = discovery.discover().await.unwrap();

    // Assert
    assert_eq!(capabilities.len(), 2);

    let orch = capabilities
        .iter()
        .find(|c| c.capability_type == "orchestration");
    assert!(orch.is_some());
    assert_eq!(orch.unwrap().endpoint, "http://orchestration:8080");

    let sec = capabilities
        .iter()
        .find(|c| c.capability_type == "security");
    assert!(sec.is_some());
    assert_eq!(sec.unwrap().endpoint, "http://security:8081");
}

/// Tests that discovery returns empty result when config has no endpoints
#[tokio::test]
async fn test_discovery_empty_config() {
    // Arrange
    let config = EnvironmentDiscoveryConfig::new(); // Empty config
    let discovery = EnvironmentDiscovery::with_config(Arc::new(config));

    // Act
    let capabilities = discovery.discover().await.unwrap();

    // Assert
    assert!(capabilities.is_empty());
}

/// Tests that metadata is correctly extracted from config
#[tokio::test]
async fn test_capability_metadata_from_config() {
    // Arrange
    let mut config = EnvironmentDiscoveryConfig::new();
    config.set_endpoint("ai", "http://ai:8082");
    config.set_metadata("ai", "auth_key", "secret-token-123");
    config.set_metadata("ai", "timeout_ms", "5000");

    let discovery = EnvironmentDiscovery::with_config(Arc::new(config));

    // Act
    let capabilities = discovery.discover().await.unwrap();

    // Assert
    let ai = capabilities
        .iter()
        .find(|c| c.capability_type == "ai")
        .unwrap();
    assert_eq!(
        ai.metadata.get("auth_key"),
        Some(&"secret-token-123".to_string())
    );
    assert_eq!(ai.metadata.get("timeout_ms"), Some(&"5000".to_string()));
}

/// Tests that confidence level is correct
#[tokio::test]
async fn test_discovery_confidence_level() {
    // Arrange
    let mut config = EnvironmentDiscoveryConfig::new();
    config.set_endpoint("storage", "http://storage:8083");

    let discovery = EnvironmentDiscovery::with_config(Arc::new(config));

    // Act
    let capabilities = discovery.discover().await.unwrap();

    // Assert
    let storage = capabilities
        .iter()
        .find(|c| c.capability_type == "storage")
        .unwrap();
    assert!(storage.confidence >= 0.9);
    assert!(storage.confidence <= 1.0);
}

// ==================== EDGE CASE TESTS ====================

/// Tests handling of extremely long endpoint URLs
#[tokio::test]
async fn test_extremely_long_endpoint_url() {
    // Arrange
    let long_url = format!("http://{}:8080/path", "a".repeat(1000));
    let mut config = EnvironmentDiscoveryConfig::new();
    config.set_endpoint("compute", &long_url);

    let discovery = EnvironmentDiscovery::with_config(Arc::new(config));

    // Act
    let capabilities = discovery.discover().await.unwrap();

    // Assert
    assert_eq!(capabilities.len(), 1);
    let compute = capabilities
        .iter()
        .find(|c| c.capability_type == "compute")
        .unwrap();
    assert_eq!(compute.endpoint, long_url);
}

/// Tests handling of special characters in endpoint URLs
#[tokio::test]
async fn test_special_characters_in_endpoint() {
    // Arrange
    let special_url = "http://service:8080/path?query=value&other=123#fragment";
    let mut config = EnvironmentDiscoveryConfig::new();
    config.set_endpoint("monitoring", special_url);

    let discovery = EnvironmentDiscovery::with_config(Arc::new(config));

    // Act
    let capabilities = discovery.discover().await.unwrap();

    // Assert
    let monitoring = capabilities
        .iter()
        .find(|c| c.capability_type == "monitoring")
        .unwrap();
    assert_eq!(monitoring.endpoint, special_url);
}

/// Tests handling of empty endpoint URL
#[tokio::test]
async fn test_empty_endpoint_url() {
    // Arrange
    let mut config = EnvironmentDiscoveryConfig::new();
    config.set_endpoint("network", "");

    let discovery = EnvironmentDiscovery::with_config(Arc::new(config));

    // Act
    let capabilities = discovery.discover().await.unwrap();

    // Assert
    let network = capabilities
        .iter()
        .find(|c| c.capability_type == "network")
        .unwrap();
    assert_eq!(network.endpoint, "");
}

/// Tests handling of multiple capabilities
#[tokio::test]
async fn test_multiple_capabilities() {
    // Arrange
    let mut config = EnvironmentDiscoveryConfig::new();
    for i in 0..50 {
        config.set_endpoint(
            &format!("service_{}", i),
            &format!("http://service-{}:8080", i),
        );
    }

    let discovery = EnvironmentDiscovery::with_config(Arc::new(config));

    // Act
    let capabilities = discovery.discover().await.unwrap();

    // Assert
    assert_eq!(capabilities.len(), 50);
}

// ==================== CONCURRENCY TESTS ====================

/// Tests that concurrent discovery calls are truly thread-safe
///
/// This test spawns 100 concurrent tasks, each using the SAME config.
/// With the old env var approach, this would have race conditions.
/// With immutable config, it's perfectly safe.
#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_concurrent_discovery_same_config() {
    // Arrange
    let mut config = EnvironmentDiscoveryConfig::new();
    config.set_endpoint("orchestration", "http://orchestration:8080");
    config.set_endpoint("security", "http://security:8081");

    let config = Arc::new(config);

    // Act - spawn 100 concurrent discovery operations
    let handles: Vec<_> = (0..100)
        .map(|_| {
            let discovery = EnvironmentDiscovery::with_config(Arc::clone(&config));
            tokio::spawn(async move { discovery.discover().await })
        })
        .collect();

    // Wait for all tasks
    let results = futures::future::join_all(handles).await;

    // Assert - all should succeed and return consistent results
    for result in results {
        let capabilities = result.unwrap().unwrap();
        assert_eq!(capabilities.len(), 2);
        assert!(capabilities
            .iter()
            .any(|c| c.capability_type == "orchestration"));
        assert!(capabilities.iter().any(|c| c.capability_type == "security"));
    }
}

/// Tests that each task can have its own independent config
///
/// This demonstrates true test isolation - each test/task can have
/// completely different configuration without interfering with others.
#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_concurrent_discovery_different_configs() {
    // Arrange - create 100 DIFFERENT configs
    let configs: Vec<_> = (0..100)
        .map(|i| {
            let mut config = EnvironmentDiscoveryConfig::new();
            config.set_endpoint("service", &format!("http://service-{}:8080", i));
            Arc::new(config)
        })
        .collect();

    // Act - spawn 100 tasks, each with different config
    let handles: Vec<_> = configs
        .into_iter()
        .enumerate()
        .map(|(i, config)| {
            tokio::spawn(async move {
                let discovery = EnvironmentDiscovery::with_config(config);
                let caps = discovery.discover().await.unwrap();
                (i, caps)
            })
        })
        .collect();

    // Wait and collect results
    let results = futures::future::join_all(handles).await;

    // Assert - each task got its own config
    for (i, result) in results.into_iter().enumerate() {
        let (task_id, capabilities) = result.unwrap();
        assert_eq!(task_id, i);
        assert_eq!(capabilities.len(), 1);

        let service = capabilities
            .iter()
            .find(|c| c.capability_type == "service")
            .unwrap();
        assert_eq!(service.endpoint, format!("http://service-{}:8080", i));
    }
}

/// Stress test: 1000 concurrent tasks
#[tokio::test(flavor = "multi_thread", worker_threads = 16)]
async fn test_high_concurrency_stress() {
    // Arrange
    let mut config = EnvironmentDiscoveryConfig::new();
    config.set_endpoint("test", "http://test:8080");
    let config = Arc::new(config);

    // Act - 1000 concurrent tasks!
    let handles: Vec<_> = (0..1000)
        .map(|_| {
            let discovery = EnvironmentDiscovery::with_config(Arc::clone(&config));
            tokio::spawn(async move { discovery.discover().await })
        })
        .collect();

    let results = futures::future::join_all(handles).await;

    // Assert - all succeed
    for result in results {
        assert!(result.unwrap().is_ok());
    }
}

// ==================== DEFAULT TRAIT TESTS ====================

/// Tests that Default trait creates valid scanner
#[tokio::test]
async fn test_default_trait() {
    // Arrange & Act
    let discovery = EnvironmentDiscovery::default();

    // Assert - should have some config (loaded from env)
    // This test doesn't check specific values since they depend on environment
    let _ = discovery.discover().await.unwrap();
}

/// Tests that new() is equivalent to from_env()
#[tokio::test]
async fn test_new_equals_from_env() {
    // Arrange & Act
    let discovery1 = EnvironmentDiscovery::new();
    let discovery2 = EnvironmentDiscovery::from_env();

    // Both should work (can't assert equality due to Arc)
    let caps1 = discovery1.discover().await.unwrap();
    let caps2 = discovery2.discover().await.unwrap();

    // Should return same results
    assert_eq!(caps1.len(), caps2.len());
}

// ==================== METHOD NAME TESTS ====================

/// Tests that method_name returns correct value
#[test]
fn test_method_name() {
    let config = Arc::new(EnvironmentDiscoveryConfig::new());
    let discovery = EnvironmentDiscovery::with_config(config);
    assert_eq!(discovery.method_name(), "environment");
}

// ==================== CAPABILITY SCANNER TESTS ====================

/// Tests that scanner aggregates results from multiple methods
#[tokio::test]
async fn test_scanner_aggregates_methods() {
    // Arrange
    let mut config = EnvironmentDiscoveryConfig::new();
    config.set_endpoint("orchestration", "http://orchestration:8080");
    config.set_endpoint("security", "http://security:8081");

    let discovery = EnvironmentDiscovery::with_config(Arc::new(config));
    let mut scanner = CapabilityScanner::new();
    scanner.add_discovery_method(DiscoveryMethodImpl::Environment(discovery));

    // Act
    let capabilities = scanner.scan_capabilities().await.unwrap();

    // Assert
    assert!(capabilities.len() >= 2);
}

/// Tests that scanner handles empty config gracefully
#[tokio::test]
async fn test_scanner_empty_config() {
    // Arrange
    let config = EnvironmentDiscoveryConfig::new(); // Empty
    let discovery = EnvironmentDiscovery::with_config(Arc::new(config));
    let mut scanner = CapabilityScanner::new();
    scanner.add_discovery_method(DiscoveryMethodImpl::Environment(discovery));

    // Act
    let result = scanner.scan_capabilities().await;

    // Assert
    assert!(result.is_ok());
}

/// Tests that scanner works with multiple discovery instances
#[tokio::test]
async fn test_scanner_multiple_discoveries() {
    // Arrange
    let mut config1 = EnvironmentDiscoveryConfig::new();
    config1.set_endpoint("storage", "http://storage:8083");

    let mut config2 = EnvironmentDiscoveryConfig::new();
    config2.set_endpoint("compute", "http://compute:8084");

    let mut scanner = CapabilityScanner::new();
    scanner.add_discovery_method(DiscoveryMethodImpl::Environment(
        EnvironmentDiscovery::with_config(Arc::new(config1)),
    ));
    scanner.add_discovery_method(DiscoveryMethodImpl::Environment(
        EnvironmentDiscovery::with_config(Arc::new(config2)),
    ));

    // Act
    let capabilities = scanner.scan_capabilities().await.unwrap();

    // Assert
    assert!(capabilities.iter().any(|c| c.capability_type == "storage"));
    assert!(capabilities.iter().any(|c| c.capability_type == "compute"));
}

// ==================== PERFORMANCE TESTS ====================

/// Tests performance of repeated scans
#[tokio::test]
async fn test_repeated_scans_performance() {
    // Arrange
    let mut config = EnvironmentDiscoveryConfig::new();
    config.set_endpoint("monitoring", "http://monitoring:8085");

    let discovery = EnvironmentDiscovery::with_config(Arc::new(config));

    // Act - perform 100 scans
    let start = std::time::Instant::now();
    for _ in 0..100 {
        let _ = discovery.discover().await.unwrap();
    }
    let duration = start.elapsed();

    // Assert - should be fast (< 100ms for 100 scans = 1ms per scan)
    assert!(
        duration.as_millis() < 100,
        "100 scans took {:?}, should be < 100ms",
        duration
    );
}

// ==================== TRAIT IMPLEMENTATION TESTS ====================

/// Tests that EnvironmentDiscovery can be cloned
#[test]
fn test_discovery_clone() {
    let mut config = EnvironmentDiscoveryConfig::new();
    config.set_endpoint("test", "http://test:8080");

    let discovery = EnvironmentDiscovery::with_config(Arc::new(config));
    let cloned = discovery.clone();

    // Both should have same config (Arc makes this cheap)
    assert_eq!(discovery.config().len(), cloned.config().len());
}

/// Tests that EnvironmentDiscovery can be debugged
#[test]
fn test_discovery_debug() {
    let config = EnvironmentDiscoveryConfig::new();
    let discovery = EnvironmentDiscovery::with_config(Arc::new(config));

    let debug_str = format!("{:?}", discovery);
    assert!(debug_str.contains("EnvironmentDiscovery"));
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
}
