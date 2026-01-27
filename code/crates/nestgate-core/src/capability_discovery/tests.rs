// Copyright (c) 2026 NestGate
//
//! Comprehensive tests for capability-based primal discovery

use super::*;
use serde_json::json;

#[test]
fn test_service_endpoint_creation() {
    let endpoint = ServiceEndpoint {
        capability: "crypto".to_string(),
        name: "crypto-provider".to_string(),
        endpoint: "/primal/crypto".to_string(),
        version: "1.0.0".to_string(),
        discovered_at: Instant::now(),
    };

    assert_eq!(endpoint.capability, "crypto");
    assert_eq!(endpoint.name, "crypto-provider");
    assert_eq!(endpoint.endpoint, "/primal/crypto");
    assert_eq!(endpoint.version, "1.0.0");
}

#[test]
fn test_service_endpoint_from_response_complete() {
    let response = json!({
        "name": "crypto-service",
        "endpoint": "/primal/crypto",
        "version": "2.0.0"
    });

    let endpoint = ServiceEndpoint::from_response(response, "crypto").unwrap();

    assert_eq!(endpoint.capability, "crypto");
    assert_eq!(endpoint.name, "crypto-service");
    assert_eq!(endpoint.endpoint, "/primal/crypto");
    assert_eq!(endpoint.version, "2.0.0");
}

#[test]
fn test_service_endpoint_from_response_missing_version() {
    let response = json!({
        "name": "storage-service",
        "endpoint": "/primal/storage"
    });

    let endpoint = ServiceEndpoint::from_response(response, "storage").unwrap();

    assert_eq!(endpoint.capability, "storage");
    assert_eq!(endpoint.name, "storage-service");
    assert_eq!(endpoint.endpoint, "/primal/storage");
    assert_eq!(endpoint.version, "unknown");
}

#[test]
fn test_service_endpoint_from_response_missing_name() {
    let response = json!({
        "endpoint": "/primal/http"
    });

    let result = ServiceEndpoint::from_response(response, "http");
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Missing service name"));
}

#[test]
fn test_service_endpoint_from_response_missing_endpoint() {
    let response = json!({
        "name": "http-service"
    });

    let result = ServiceEndpoint::from_response(response, "http");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Missing endpoint"));
}

// Note: Tests that require JsonRpcClient are integration tests
// and require Songbird IPC to be running. See integration tests below.

#[test]
fn test_service_endpoint_clone() {
    let endpoint1 = ServiceEndpoint {
        capability: "crypto".to_string(),
        name: "crypto-provider".to_string(),
        endpoint: "/primal/crypto".to_string(),
        version: "1.0.0".to_string(),
        discovered_at: Instant::now(),
    };

    let endpoint2 = endpoint1.clone();

    assert_eq!(endpoint1.capability, endpoint2.capability);
    assert_eq!(endpoint1.name, endpoint2.name);
    assert_eq!(endpoint1.endpoint, endpoint2.endpoint);
    assert_eq!(endpoint1.version, endpoint2.version);
}

#[test]
fn test_service_endpoint_serialization() {
    let endpoint = ServiceEndpoint {
        capability: "crypto".to_string(),
        name: "crypto-provider".to_string(),
        endpoint: "/primal/crypto".to_string(),
        version: "1.0.0".to_string(),
        discovered_at: Instant::now(),
    };

    // Serialize to JSON
    let json_str = serde_json::to_string(&endpoint).unwrap();

    // Should contain all fields except discovered_at (which is skipped)
    assert!(json_str.contains("crypto"));
    assert!(json_str.contains("crypto-provider"));
    assert!(json_str.contains("/primal/crypto"));
    assert!(json_str.contains("1.0.0"));
    assert!(!json_str.contains("discovered_at"));
}

#[test]
fn test_service_endpoint_deserialization() {
    let json_str = r#"{
        "capability": "http",
        "name": "http-service",
        "endpoint": "/primal/http",
        "version": "2.1.0"
    }"#;

    let endpoint: ServiceEndpoint = serde_json::from_str(json_str).unwrap();

    assert_eq!(endpoint.capability, "http");
    assert_eq!(endpoint.name, "http-service");
    assert_eq!(endpoint.endpoint, "/primal/http");
    assert_eq!(endpoint.version, "2.1.0");
    // discovered_at should be set to now() via default
}

#[test]
fn test_cache_stats_debug() {
    let stats = CacheStats {
        size: 5,
        ttl_seconds: 300,
    };

    let debug_str = format!("{:?}", stats);
    assert!(debug_str.contains("size"));
    assert!(debug_str.contains("5"));
    assert!(debug_str.contains("ttl_seconds"));
    assert!(debug_str.contains("300"));
}

#[test]
fn test_cache_stats_clone() {
    let stats1 = CacheStats {
        size: 10,
        ttl_seconds: 600,
    };

    let stats2 = stats1.clone();

    assert_eq!(stats1.size, stats2.size);
    assert_eq!(stats1.ttl_seconds, stats2.ttl_seconds);
}

// Integration tests (require Songbird IPC to be running)
// These are marked as ignored by default
#[tokio::test]
#[ignore = "Requires Songbird IPC service running"]
async fn test_discover_songbird_ipc_integration() {
    // This test requires Songbird to be running at /primal/songbird
    let result = CapabilityDiscovery::discover_songbird_ipc().await;

    // Should either succeed or fail with specific error message
    match result {
        Ok(_client) => {
            // Success: Songbird is running
        }
        Err(e) => {
            // Expected error message when Songbird is not running
            assert!(e.to_string().contains("Songbird IPC not found"));
        }
    }
}

#[tokio::test]
#[ignore = "Requires Songbird IPC service running"]
async fn test_find_capability_integration() {
    // This test requires Songbird to be running and have services registered
    let songbird = match CapabilityDiscovery::discover_songbird_ipc().await {
        Ok(client) => client,
        Err(_) => return, // Skip test if Songbird not available
    };

    let mut discovery = CapabilityDiscovery::new(songbird);

    // Try to discover HTTP capability (Songbird itself)
    let result = discovery.find("http").await;

    match result {
        Ok(endpoint) => {
            assert_eq!(endpoint.capability, "http");
            assert!(!endpoint.endpoint.is_empty());
        }
        Err(e) => {
            // Expected if no service provides HTTP capability
            assert!(e.to_string().contains("No service provides capability"));
        }
    }
}

#[tokio::test]
#[ignore = "Requires Songbird IPC service running"]
async fn test_cache_hit_integration() {
    let songbird = match CapabilityDiscovery::discover_songbird_ipc().await {
        Ok(client) => client,
        Err(_) => return,
    };

    let mut discovery = CapabilityDiscovery::new(songbird);

    // First call - should query Songbird
    let result1 = discovery.find("http").await;

    // Second call - should hit cache
    let result2 = discovery.find("http").await;

    // Both should return same result
    if let (Ok(endpoint1), Ok(endpoint2)) = (result1, result2) {
        assert_eq!(endpoint1.capability, endpoint2.capability);
        assert_eq!(endpoint1.endpoint, endpoint2.endpoint);
    }
}
