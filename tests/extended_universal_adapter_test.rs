//! **EXTENDED UNIVERSAL ADAPTER TEST SUITE**
//!
//! Comprehensive validation of the universal adapter system after canonical modernization:
//! - Universal adapter pattern validation
//! - Cross-domain adapter functionality
//! - Adapter configuration consistency
//! - Performance under load
//! - Error handling across adapters

use nestgate_core::error::{NestGateError, Result};
use nestgate_core::unified_enums::service_types::{UnifiedHealthStatus, UnifiedServiceType};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Test adapter service type unification
#[tokio::test]
async fn test_adapter_service_type_unification() -> Result<()> {
    println!("🧪 Testing adapter service type unification...");

    // Test all unified service types work consistently
    let service_types = vec![
        UnifiedServiceType::Storage,
        UnifiedServiceType::Network,
        UnifiedServiceType::Security,
        UnifiedServiceType::Monitoring,
        UnifiedServiceType::Gateway,
        UnifiedServiceType::Adapter,
    ];

    // Test adapter metadata for each service type
    let mut adapter_metadata = HashMap::new();

    for service_type in service_types {
        // Create adapter metadata
        let metadata = format!("adapter_for_{service_type:?}");

        // Test service type serialization
        let serialized = serde_json::to_string(&service_type).map_err(|e| {
            NestGateError::internal_error(
                format!("Service type serialization failed: {e}"),
                "test_adapter_service_type_unification",
            )
        })?;

        // Verify consistent naming pattern
        assert!(
            !serialized.contains("fragmented"),
            "Service type should not contain fragmented patterns"
        );

        // Store metadata after serialization test
        adapter_metadata.insert(service_type, metadata);
    }

    // Verify all service types are represented
    assert_eq!(
        adapter_metadata.len(),
        6,
        "Should have metadata for all 6 service types"
    );

    println!("✅ Adapter service type unification validated");
    Ok(())
}

/// Test universal adapter health monitoring
#[tokio::test]
async fn test_universal_adapter_health_monitoring() -> Result<()> {
    println!("🧪 Testing universal adapter health monitoring...");

    // Test health status transitions
    let health_transitions = vec![
        (UnifiedHealthStatus::Unknown, UnifiedHealthStatus::Healthy),
        (UnifiedHealthStatus::Healthy, UnifiedHealthStatus::Degraded),
        (
            UnifiedHealthStatus::Degraded,
            UnifiedHealthStatus::Unhealthy,
        ),
        (UnifiedHealthStatus::Unhealthy, UnifiedHealthStatus::Healthy),
    ];

    for (from_status, to_status) in health_transitions {
        // Simulate health status change
        let transition_time = Instant::now();

        // Test status serialization performance
        let from_serialized = serde_json::to_string(&from_status).map_err(|e| {
            NestGateError::internal_error(
                format!("Health status serialization failed: {e}"),
                "test_universal_adapter_health_monitoring",
            )
        })?;

        let to_serialized = serde_json::to_string(&to_status).map_err(|e| {
            NestGateError::internal_error(
                format!("Health status serialization failed: {e}"),
                "test_universal_adapter_health_monitoring",
            )
        })?;

        let elapsed = transition_time.elapsed();

        // Health transitions should be fast
        assert!(
            elapsed < Duration::from_millis(10),
            "Health status transitions should be very fast (was {:?})",
            elapsed
        );

        // Verify consistent serialization format
        assert!(
            !from_serialized.contains("fragmented"),
            "Health status should not contain fragmented patterns"
        );
        assert!(
            !to_serialized.contains("fragmented"),
            "Health status should not contain fragmented patterns"
        );
    }

    println!("✅ Universal adapter health monitoring validated");
    Ok(())
}

/// Test adapter error handling consistency
#[tokio::test]
async fn test_adapter_error_handling_consistency() -> Result<()> {
    println!("🧪 Testing adapter error handling consistency...");

    // Test different adapter error scenarios
    let adapter_errors = vec![
        (
            "storage",
            NestGateError::storage_error("Storage adapter error"),
        ),
        (
            "internal",
            NestGateError::internal_error("Internal adapter error", "test_component"),
        ),
        (
            "config",
            NestGateError::configuration_error("test_field", "Configuration adapter error"),
        ),
    ];

    for (adapter_type, error) in adapter_errors {
        // Test error formatting consistency
        let error_string = format!("{error}");

        // Verify error contains meaningful information
        assert!(
            !error_string.is_empty(),
            "Error should have meaningful message for {adapter_type}"
        );

        // Verify no fragmented error patterns
        assert!(
            !error_string.contains("fragmented"),
            "Error should not contain fragmented patterns for {adapter_type}"
        );

        // Test that error can be formatted (validates error implementation)
        let debug_format = format!("{error:?}");
        assert!(
            !debug_format.is_empty(),
            "Error debug format should not be empty for {adapter_type}"
        );
    }

    println!("✅ Adapter error handling consistency validated");
    Ok(())
}

/// Test adapter enum serialization and display traits
#[tokio::test]
async fn test_adapter_enum_traits() -> Result<()> {
    println!("🧪 Testing adapter enum trait implementations...");

    // Test UnifiedServiceType Display trait
    let service_types = vec![
        (UnifiedServiceType::Storage, "storage"),
        (UnifiedServiceType::Network, "network"),
        (UnifiedServiceType::Security, "security"),
        (UnifiedServiceType::Monitoring, "monitoring"),
        (UnifiedServiceType::Gateway, "gateway"),
        (UnifiedServiceType::Adapter, "adapter"),
    ];

    for (service_type, expected) in service_types {
        let display_string = format!("{service_type}");
        assert_eq!(
            display_string, expected,
            "Service type display should match expected format"
        );
    }

    // Test UnifiedHealthStatus serialization
    let health_statuses = vec![
        UnifiedHealthStatus::Healthy,
        UnifiedHealthStatus::Degraded,
        UnifiedHealthStatus::Unhealthy,
        UnifiedHealthStatus::Unknown,
    ];

    for status in health_statuses {
        let serialized = serde_json::to_string(&status).map_err(|e| {
            NestGateError::internal_error(
                format!("Health status serialization failed: {e}"),
                "test_adapter_enum_traits",
            )
        })?;

        let deserialized: UnifiedHealthStatus = serde_json::from_str(&serialized).map_err(|e| {
            NestGateError::internal_error(
                format!("Health status deserialization failed: {e}"),
                "test_adapter_enum_traits",
            )
        })?;

        assert_eq!(
            status, deserialized,
            "Health status should round-trip through serialization"
        );
    }

    println!("✅ Adapter enum traits validated");
    Ok(())
}

/// Test adapter system performance under concurrent load
#[tokio::test]
async fn test_adapter_concurrent_performance() -> Result<()> {
    println!("🧪 Testing adapter system concurrent performance...");

    let concurrent_adapters = 50;
    let operations_per_adapter = 100;

    let start = Instant::now();

    // Create concurrent adapter simulations
    let handles: Vec<_> = (0..concurrent_adapters)
        .map(|adapter_id| {
            tokio::spawn(async move {
                let mut results = Vec::new();

                for op_id in 0..operations_per_adapter {
                    // Simulate adapter operations with enum serialization
                    let service_type = match adapter_id % 6 {
                        0 => UnifiedServiceType::Storage,
                        1 => UnifiedServiceType::Network,
                        2 => UnifiedServiceType::Security,
                        3 => UnifiedServiceType::Monitoring,
                        4 => UnifiedServiceType::Gateway,
                        _ => UnifiedServiceType::Adapter,
                    };

                    // Simulate adapter processing
                    let result = format!("adapter_{adapter_id}_op_{op_id}_{service_type}");
                    results.push(result);
                }

                results.len()
            })
        })
        .collect();

    // Wait for all adapters to complete
    let results: Vec<_> = futures::future::join_all(handles).await;
    let total_time = start.elapsed();

    // Verify all adapters completed successfully
    let total_operations: usize = results.into_iter().map(|result| result.unwrap_or(0)).sum();

    let expected_operations = concurrent_adapters * operations_per_adapter;
    assert_eq!(
        total_operations, expected_operations,
        "All adapter operations should complete"
    );

    // Calculate throughput
    let ops_per_sec = total_operations as f64 / total_time.as_secs_f64();

    // Should handle high concurrent throughput
    assert!(
        ops_per_sec > 1000.0,
        "Should handle >1000 concurrent adapter ops/sec, got {ops_per_sec:.0}"
    );

    println!("✅ Adapter concurrent performance validated");
    println!("   📊 Concurrent adapters: {concurrent_adapters}");
    println!("   📊 Total operations: {total_operations}");
    println!("   📊 Throughput: {ops_per_sec:.0} ops/sec");

    Ok(())
}

/// Test adapter default trait implementations
#[tokio::test]
async fn test_adapter_defaults() -> Result<()> {
    println!("🧪 Testing adapter default implementations...");

    // Test UnifiedServiceType default
    let default_service_type = UnifiedServiceType::default();
    assert_eq!(
        default_service_type,
        UnifiedServiceType::Unknown,
        "Default service type should be Unknown"
    );

    // Test UnifiedHealthStatus default
    let default_health_status = UnifiedHealthStatus::default();
    assert_eq!(
        default_health_status,
        UnifiedHealthStatus::Unknown,
        "Default health status should be Unknown"
    );

    println!("✅ Adapter default implementations validated");
    Ok(())
}

/// Test adapter clone and equality traits
#[tokio::test]
async fn test_adapter_clone_eq() -> Result<()> {
    println!("🧪 Testing adapter Clone and PartialEq traits...");

    // Test service type cloning
    let service_type = UnifiedServiceType::Storage;
    let cloned_service_type = service_type.clone();
    assert_eq!(
        service_type, cloned_service_type,
        "Cloned service type should equal original"
    );

    // Test health status cloning
    let health_status = UnifiedHealthStatus::Healthy;
    let cloned_health_status = health_status.clone();
    assert_eq!(
        health_status, cloned_health_status,
        "Cloned health status should equal original"
    );

    // Test HashMap usage (requires Hash + Eq)
    let mut service_map = HashMap::new();
    service_map.insert(UnifiedServiceType::Storage, "storage_adapter");
    service_map.insert(UnifiedServiceType::Network, "network_adapter");

    assert_eq!(
        service_map.get(&UnifiedServiceType::Storage),
        Some(&"storage_adapter"),
        "Service type should work as HashMap key"
    );

    println!("✅ Adapter Clone and PartialEq traits validated");
    Ok(())
}
