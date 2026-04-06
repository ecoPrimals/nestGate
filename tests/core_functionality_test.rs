// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![allow(
    unused,
    dead_code,
    deprecated,
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::restriction,
    clippy::cargo
)]

//! Core functionality tests for NestGate
//! Tests the core functionality that compiles and works correctly
//!
//! **MODERN CONCURRENCY**: Uses yield_now() for async coordination instead of sleep().

use nestgate_core::{
    canonical_modernization::unified_enums::{UnifiedHealthStatus, UnifiedServiceType},
    config::canonical_primary::NestGateCanonicalConfig,
    error::{NestGateError, Result},
    service_discovery::types::{
        CommunicationProtocol, ServiceCapability, ServiceCategory, ServiceEndpoint, ServiceInfo,
        ServiceMetadata, StorageType,
    },
};
use std::collections::HashMap;
use std::time::SystemTime;
use uuid::Uuid;

#[tokio::test]
async fn test_canonical_config_creation() -> Result<()> {
    println!("🧪 Testing canonical configuration creation");

    // Test default configuration with explicit type
    let _config: NestGateCanonicalConfig = NestGateCanonicalConfig::default();
    println!("✅ Created default canonical config");

    Ok(())
}

#[tokio::test]
async fn test_unified_enums() -> Result<()> {
    println!("🧪 Testing unified enum system");

    // Test service types
    let _storage_service = UnifiedServiceType::Storage;
    let _network_service = UnifiedServiceType::Network;
    let _security_service = UnifiedServiceType::Security;

    println!("✅ Service types created successfully");

    // Test health status
    let _healthy = UnifiedHealthStatus::Healthy;
    let _unhealthy = UnifiedHealthStatus::Unhealthy;

    println!("✅ Health status enums created successfully");

    Ok(())
}

#[tokio::test]
async fn test_error_system() -> Result<()> {
    println!("🧪 Testing unified error system");

    // Test creating different types of errors
    let internal_error = NestGateError::internal_error(
        "Test internal error".to_string(),
        "test_context".to_string(),
    );

    println!("✅ Created internal error");

    // Test error conversion
    let result: Result<String> = Err(internal_error);
    match result {
        Ok(_) => unreachable!(),
        Err(_) => {
            println!("✅ Error handling works correctly");
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_service_discovery_types() -> Result<()> {
    println!("🧪 Testing service discovery type system");

    // Test creating service metadata
    let metadata = ServiceMetadata {
        name: "test-service".to_string(),
        category: ServiceCategory::Storage,
        version: "1.0.0".to_string(),
        description: "Test service".to_string(),
        health_endpoint: Some("http://localhost:8080/health".to_string()),
        metrics_endpoint: None,
    };

    // Test creating service endpoint
    let endpoint = ServiceEndpoint {
        url: "http://localhost:8080".to_string(),
        protocol: CommunicationProtocol::Http,
        health_check: Some("/health".to_string()),
    };

    // Test creating service info
    let service_info = ServiceInfo {
        service_id: Uuid::new_v4(),
        metadata,
        capabilities: vec![ServiceCapability::Storage(StorageType::FileSystem)],
        endpoints: vec![endpoint],
        last_seen: SystemTime::now(),
    };

    println!(
        "✅ Created service info with ID: {}",
        service_info.service_id
    );

    // Test service capabilities
    let storage_cap = ServiceCapability::Storage(StorageType::FileSystem);
    println!("✅ Created storage capability: {:?}", storage_cap);

    Ok(())
}

#[tokio::test]
async fn test_async_operations() -> Result<()> {
    println!("🧪 Testing async operations");

    // Test async coordination
    let start = SystemTime::now();
    tokio::task::yield_now().await;
    let elapsed = start
        .elapsed()
        .map_err(|e| NestGateError::internal_error(e.to_string(), "test_async_operations"))?;

    println!("✅ Async coordination worked: {elapsed:?}");

    // Test async error handling
    async fn async_operation_that_fails() -> Result<String> {
        Err(NestGateError::internal_error(
            "Async operation failed".to_string(),
            "test".to_string(),
        ))
    }

    match async_operation_that_fails().await {
        Ok(_) => unreachable!(),
        Err(_) => {
            println!("✅ Async error handling works");
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_concurrent_operations() -> Result<()> {
    println!("🧪 Testing concurrent operations");

    let mut handles = Vec::new();

    // Spawn multiple concurrent tasks
    for i in 0..5 {
        let handle = tokio::spawn(async move {
            tokio::task::yield_now().await;
            format!("Task {i} completed")
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    let mut results = Vec::new();
    for handle in handles {
        let result = handle.await.map_err(|e| {
            NestGateError::internal_error(
                format!("Task join failed: {e}"),
                "test_concurrent_operations".to_string(),
            )
        })?;
        results.push(result);
    }

    assert_eq!(results.len(), 5, "All tasks should complete");
    println!(
        "✅ Concurrent operations completed: {} tasks",
        results.len()
    );

    Ok(())
}

#[tokio::test]
async fn test_json_serialization() -> Result<()> {
    println!("🧪 Testing JSON serialization");

    // Test service metadata serialization
    let metadata = ServiceMetadata {
        name: "test-service".to_string(),
        category: ServiceCategory::Storage,
        version: "1.0.0".to_string(),
        description: "Test service for JSON".to_string(),
        health_endpoint: Some("http://localhost:8080/health".to_string()),
        metrics_endpoint: Some("http://localhost:8080/metrics".to_string()),
    };

    // Serialize to JSON
    let json_str = serde_json::to_string(&metadata).map_err(|e| {
        NestGateError::internal_error(
            format!("JSON serialization failed: {e}"),
            "test_json_serialization".to_string(),
        )
    })?;

    println!("✅ Metadata serialized to JSON: {} bytes", json_str.len());

    // Deserialize from JSON
    let deserialized: ServiceMetadata = serde_json::from_str(&json_str).map_err(|e| {
        NestGateError::internal_error(
            format!("JSON deserialization failed: {e}"),
            "test_json_serialization".to_string(),
        )
    })?;

    assert_eq!(deserialized.name, "test-service");
    assert_eq!(deserialized.version, "1.0.0");

    println!("✅ Metadata deserialized successfully");

    Ok(())
}

#[tokio::test]
async fn test_memory_operations() -> Result<()> {
    println!("🧪 Testing memory operations");

    // Test creating and manipulating data structures
    let mut data_map: HashMap<String, Vec<String>> = HashMap::new();

    for i in 0..100 {
        let key = format!("key_{i}");
        let values = vec![
            format!("value_{}_a", i),
            format!("value_{}_b", i),
            format!("value_{}_c", i),
        ];
        data_map.insert(key, values);
    }

    assert_eq!(data_map.len(), 100);

    // Test accessing data
    let key_50 = "key_50".to_string();
    if let Some(values) = data_map.get(&key_50) {
        assert_eq!(values.len(), 3);
        assert_eq!(values[0], "value_50_a");
    } else {
        return Err(NestGateError::internal_error(
            "Expected key not found".to_string(),
            "test_memory_operations".to_string(),
        ));
    }

    println!("✅ Memory operations completed successfully");

    Ok(())
}
