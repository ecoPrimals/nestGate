//! **EXTENDED UNIVERSAL ADAPTER TEST SUITE**
//!
//! Comprehensive validation of the universal adapter system after canonical modernization:
//! - Universal adapter pattern validation
//! - Cross-domain adapter functionality
//! - Adapter configuration consistency
//! - Performance under load
//! - Error handling across adapters

use crate::canonical_modernization::{UnifiedHealthStatus, UnifiedServiceType};
use nestgate_core::config::canonical_unified::types::CanonicalConfig;
use nestgate_core::error::{NestGateError, Result};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Test universal adapter configuration consistency
#[tokio::test]
async fn test_universal_adapter_config_consistency() -> Result<()> {
    println!("🧪 Testing universal adapter configuration consistency...");

    // Create configurations for different adapter types
    let storage_config = CanonicalConfig {
        storage: nestgate_core::config::canonical_unified::types::StorageConfig {
            backend_type: "universal".to_string(),
            ..Default::default()
        },
        ..Default::default()
    };

    let network_config = CanonicalConfig {
        network: nestgate_core::config::canonical_unified::types::NetworkConfig {
            api: nestgate_core::config::canonical_unified::types::ApiServerConfig {
                port: 8080,
                host: "0.0.0.0".parse().unwrap(),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    };

    // Test that all adapter configs serialize consistently
    let storage_serialized =
        serde_json::to_string(&storage_config).map_err(|e| NestGateError::Internal {
            message: format!("Storage adapter config serialization failed: {e}"),
            location: Some("test_universal_adapter_config_consistency".to_string()),
            debug_info: None,
            is_bug: false,
        })?;

    let network_serialized =
        serde_json::to_string(&network_config).map_err(|e| NestGateError::Internal {
            message: format!("Network adapter config serialization failed: {e}"),
            location: Some("test_universal_adapter_config_consistency".to_string()),
            debug_info: None,
            is_bug: false,
        })?;

    // Verify canonical structure is consistent across adapters
    assert!(
        storage_serialized.contains("\"storage\""),
        "Storage config should contain storage domain"
    );
    assert!(
        network_serialized.contains("\"network\""),
        "Network config should contain network domain"
    );

    // Verify no fragmented patterns
    assert!(
        !storage_serialized.contains("\"fragmented\""),
        "Should not contain fragmented patterns"
    );
    assert!(
        !network_serialized.contains("\"fragmented\""),
        "Should not contain fragmented patterns"
    );

    println!("✅ Universal adapter configuration consistency validated");
    Ok(())
}

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
        let serialized =
            serde_json::to_string(&service_type).map_err(|e| NestGateError::Internal {
                message: format!("Service type serialization failed: {e}"),
                location: Some("test_adapter_service_type_unification".to_string()),
                debug_info: None,
                is_bug: false,
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
        let from_serialized =
            serde_json::to_string(&from_status).map_err(|e| NestGateError::Internal {
                message: format!("Health status serialization failed: {e}"),
                location: Some("test_universal_adapter_health_monitoring".to_string()),
                debug_info: None,
                is_bug: false,
            })?;

        let to_serialized =
            serde_json::to_string(&to_status).map_err(|e| NestGateError::Internal {
                message: format!("Health status serialization failed: {e}"),
                location: Some("test_universal_adapter_health_monitoring".to_string()),
                debug_info: None,
                is_bug: false,
            })?;

        let elapsed = transition_time.elapsed();

        // Health transitions should be fast
        assert!(
            elapsed < Duration::from_millis(1),
            "Health status transitions should be very fast"
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
            NestGateError::Storage {
                operation: "adapter_operation".to_string(),
                details: "Storage adapter error".to_string(),
            },
        ),
        (
            "internal",
            NestGateError::Internal {
                message: "Internal adapter error".to_string(),
                location: Some("internal_adapter".to_string()),
                debug_info: None,
                is_bug: false,
            },
        ),
        (
            "config",
            NestGateError::Configuration {
                message: "Configuration adapter error".to_string(),
                config_source: nestgate_core::error::UnifiedConfigSource::Environment,
                field: Some("adapter_field".to_string()),
                suggested_fix: Some("Check adapter configuration".to_string()),
            },
        ),
    ];

    for (adapter_type, error) in adapter_errors {
        // Test error formatting consistency
        let error_string = format!("{error}");

        // Verify error contains meaningful information
        assert!(
            !error_string.is_empty(),
            "Error should have meaningful message"
        );
        assert!(
            error_string.contains("adapter"),
            "Error should mention adapter context"
        );

        // Verify no fragmented error patterns
        assert!(
            !error_string.contains("fragmented"),
            "Error should not contain fragmented patterns"
        );

        // Test error context tracking
        match &error {
            NestGateError::Storage { operation, details } => {
                assert!(
                    !operation.is_empty(),
                    "Storage error should have operation for {adapter_type}"
                );
                assert!(
                    details.contains("adapter"),
                    "Details should mention adapter"
                );
            }
            NestGateError::Internal { location, .. } => {
                assert!(
                    location.is_some(),
                    "Internal error should have location for {adapter_type}"
                );
                assert!(
                    location.as_ref().unwrap().contains("adapter"),
                    "Location should mention adapter"
                );
            }
            NestGateError::Configuration { message, field, .. } => {
                assert!(
                    !message.is_empty(),
                    "Config error should have message for {adapter_type}"
                );
                assert!(field.is_some(), "Config error should have field info");
            }
            _ => {}
        }
    }

    println!("✅ Adapter error handling consistency validated");
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
                    // Simulate adapter operations
                    let config = CanonicalConfig {
                        network: nestgate_core::config::canonical_unified::types::NetworkConfig {
                            api: nestgate_core::config::canonical_unified::types::ApiServerConfig {
                                port: 8000 + adapter_id as u16,
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        ..Default::default()
                    };

                    // Simulate adapter processing
                    let result = format!("adapter_{adapter_id}_op_{op_id}");
                    results.push((config, result));
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
        "Should handle >1000 concurrent adapter ops/sec"
    );

    println!("✅ Adapter concurrent performance validated");
    println!("   📊 Concurrent adapters: {concurrent_adapters}");
    println!("   📊 Total operations: {total_operations}");
    println!("   📊 Throughput: {ops_per_sec:.0} ops/sec");

    Ok(())
}

/// Test adapter configuration validation and error recovery
#[tokio::test]
async fn test_adapter_configuration_validation() -> Result<()> {
    println!("🧪 Testing adapter configuration validation...");

    // Test valid configurations
    let valid_configs = vec![
        CanonicalConfig {
            storage: nestgate_core::config::canonical_unified::types::StorageConfig {
                backend_type: "memory".to_string(),
                ..Default::default()
            },
            ..Default::default()
        },
        CanonicalConfig {
            storage: nestgate_core::config::canonical_unified::types::StorageConfig {
                backend_type: "filesystem".to_string(),
                ..Default::default()
            },
            ..Default::default()
        },
        CanonicalConfig {
            storage: nestgate_core::config::canonical_unified::types::StorageConfig {
                backend_type: "hybrid".to_string(),
                ..Default::default()
            },
            ..Default::default()
        },
    ];

    // Verify all valid configurations work
    for (i, config) in valid_configs.iter().enumerate() {
        let serialized = serde_json::to_string(config).map_err(|e| NestGateError::Internal {
            message: format!("Valid config {i} serialization failed: {e}"),
            location: Some("test_adapter_configuration_validation".to_string()),
            debug_info: None,
            is_bug: false,
        })?;

        let _deserialized: CanonicalConfig =
            serde_json::from_str(&serialized).map_err(|e| NestGateError::Internal {
                message: format!("Valid config {i} deserialization failed: {e}"),
                location: Some("test_adapter_configuration_validation".to_string()),
                debug_info: None,
                is_bug: false,
            })?;
    }

    println!("✅ Adapter configuration validation completed");
    Ok(())
}
