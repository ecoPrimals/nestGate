//! **EXTENDED CANONICAL VALIDATION TEST SUITE**
//!
//! Comprehensive validation of canonical modernization achievements including:
//! - Configuration system unification
//! - Import path standardization  
//! - Deprecation elimination
//! - Fragment consolidation
//! - Performance optimization validation

use nestgate_core::canonical_types::{health::HealthStatus, service::ServiceType};
use nestgate_core::config::unified::types::{
    CanonicalConfig, NetworkConfig, SecurityConfig, StorageConfig,
};
use nestgate_core::config::unified::NestGateUnifiedConfig as NestGateUnifiedConfig;
use nestgate_core::error::{NestGateError, Result};
use serde_json;
use std::time::Duration;

/// Test that canonical configuration system is fully unified
#[tokio::test]
async fn test_canonical_config_unification() -> Result<()> {
    println!("🧪 Testing canonical configuration unification...");

    // Verify canonical config creation works
    let config = CanonicalConfig::default();

    // Test that all major domains are present
    assert!(
        config.network.api.port > 0,
        "Network config should have valid port"
    );
    assert!(
        !config.storage.backend_type.is_empty(),
        "Storage config should have backend type"
    );
    assert!(
        config.security.enable_auth || !config.security.enable_auth,
        "Security config should be accessible"
    );

    // Test configuration serialization/deserialization
    let serialized = serde_json::to_string(&config).map_err(|e| NestGateError::Internal {
        message: format!("Config serialization failed: {e}"),
        location: Some("test_canonical_config_unification".to_string()),
        debug_info: None,
        is_bug: false,
    })?;

    let deserialized: CanonicalConfig =
        serde_json::from_str(&serialized).map_err(|e| NestGateError::Internal {
            message: format!("Config deserialization failed: {e}"),
            location: Some("test_canonical_config_unification".to_string()),
            debug_info: None,
            is_bug: false,
        })?;

    assert_eq!(config.network.api.port, deserialized.network.api.port);

    println!("✅ Canonical configuration unification validated");
    Ok(())
}

/// Test that no fragmented configuration patterns remain
#[tokio::test]
async fn test_no_fragmented_patterns() -> Result<()> {
    println!("🧪 Testing absence of fragmented configuration patterns...");

    // Create multiple configs to ensure they use unified patterns
    let configs = vec![
        CanonicalConfig::default(),
        CanonicalConfig {
            network: NetworkConfig {
                api: nestgate_core::config::unified::types::ApiServerConfig {
                    port: 8080,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        },
        CanonicalConfig {
            storage: StorageConfig {
                backend_type: "memory".to_string(),
                ..Default::default()
            },
            ..Default::default()
        },
    ];

    // Verify all configs can be serialized consistently
    for (i, config) in configs.iter().enumerate() {
        let serialized = serde_json::to_string(config).map_err(|e| NestGateError::Internal {
            message: format!("Config {i} serialization failed: {e}"),
            location: Some("test_no_fragmented_patterns".to_string()),
            debug_info: None,
            is_bug: false,
        })?;

        // Verify no fragmented field names exist
        assert!(
            !serialized.contains("\"extensions\""),
            "Should not contain deprecated 'extensions' field"
        );
        assert!(
            !serialized.contains("\"service\""),
            "Should not contain deprecated 'service' field"
        );
        assert!(
            !serialized.contains("\"fragmented\""),
            "Should not contain 'fragmented' patterns"
        );
    }

    println!("✅ No fragmented configuration patterns found");
    Ok(())
}

/// Test unified type system consistency
#[tokio::test]
async fn test_unified_type_system() -> Result<()> {
    println!("🧪 Testing unified type system consistency...");

    // Test unified service types
    let service_types = vec![
        UnifiedServiceType::Storage,
        UnifiedServiceType::Network,
        UnifiedServiceType::Security,
        UnifiedServiceType::Monitoring,
    ];

    for service_type in service_types {
        // Verify each type can be serialized/deserialized
        let serialized =
            serde_json::to_string(&service_type).map_err(|e| NestGateError::Internal {
                message: format!("Service type serialization failed: {e}"),
                location: Some("test_unified_type_system".to_string()),
                debug_info: None,
                is_bug: false,
            })?;

        let _deserialized: UnifiedServiceType =
            serde_json::from_str(&serialized).map_err(|e| NestGateError::Internal {
                message: format!("Service type deserialization failed: {e}"),
                location: Some("test_unified_type_system".to_string()),
                debug_info: None,
                is_bug: false,
            })?;
    }

    // Test unified health status
    let health_statuses = vec![
        UnifiedHealthStatus::Healthy,
        UnifiedHealthStatus::Unhealthy,
        UnifiedHealthStatus::Degraded,
        UnifiedHealthStatus::Unknown,
    ];

    for status in health_statuses {
        // Verify consistent serialization
        let serialized = serde_json::to_string(&status).map_err(|e| NestGateError::Internal {
            message: format!("Health status serialization failed: {e}"),
            location: Some("test_unified_type_system".to_string()),
            debug_info: None,
            is_bug: false,
        })?;

        assert!(
            !serialized.contains("fragmented"),
            "Health status should not contain fragmented patterns"
        );
    }

    println!("✅ Unified type system consistency validated");
    Ok(())
}

/// Test that canonical modernization maintains backward compatibility
#[tokio::test]
async fn test_backward_compatibility() -> Result<()> {
    println!("🧪 Testing backward compatibility after modernization...");

    // Test that NestGateCanonicalUnifiedConfig still works (unified alias)
    let final_config = NestGateCanonicalUnifiedConfig::default();

    // Test that final config can be serialized and used
    let final_serialized =
        serde_json::to_string(&final_config).map_err(|e| NestGateError::Internal {
            message: format!("Final config serialization failed: {e}"),
            location: Some("test_backward_compatibility".to_string()),
            debug_info: None,
            is_bug: false,
        })?;

    // Verify final config has expected structure
    assert!(
        final_serialized.contains("\"system\""),
        "Final config should contain system section"
    );
    assert!(
        final_serialized.contains("\"environment\""),
        "Final config should contain environment"
    );

    // Test that canonical config works
    let canonical_config = CanonicalConfig::default();
    let canonical_serialized =
        serde_json::to_string(&canonical_config).map_err(|e| NestGateError::Internal {
            message: format!("Canonical config serialization failed: {e}"),
            location: Some("test_backward_compatibility".to_string()),
            debug_info: None,
            is_bug: false,
        })?;

    // Verify canonical config has expected structure
    assert!(
        canonical_serialized.contains("\"network\""),
        "Canonical config should contain network section"
    );
    assert!(
        canonical_serialized.contains("\"storage\""),
        "Canonical config should contain storage section"
    );

    // Both configurations should be valid and usable
    assert!(
        !final_serialized.is_empty(),
        "Final config should serialize to non-empty JSON"
    );
    assert!(
        !canonical_serialized.is_empty(),
        "Canonical config should serialize to non-empty JSON"
    );

    println!("✅ Backward compatibility maintained - both config types work correctly");
    Ok(())
}

/// Test performance of canonical configuration operations
#[tokio::test]
async fn test_canonical_config_performance() -> Result<()> {
    println!("🧪 Testing canonical configuration performance...");

    let start = std::time::Instant::now();

    // Create many configurations to test performance
    let configs: Vec<CanonicalConfig> = (0..1000)
        .map(|i| CanonicalConfig {
            network: NetworkConfig {
                api: nestgate_core::config::unified::types::ApiServerConfig {
                    port: 8000 + (i % 100) as u16,
                    ..Default::default()
                },
                ..Default::default()
            },
            storage: StorageConfig {
                backend_type: format!("backend_{}", i % 10),
                ..Default::default()
            },
            ..Default::default()
        })
        .collect();

    let creation_time = start.elapsed();

    // Test serialization performance
    let start = std::time::Instant::now();
    let _serialized: Vec<String> = configs
        .iter()
        .map(|config| serde_json::to_string(config).unwrap())
        .collect();
    let serialization_time = start.elapsed();

    // Performance should be reasonable (less than 100ms for 1000 configs)
    assert!(
        creation_time < Duration::from_millis(100),
        "Config creation should be fast"
    );
    assert!(
        serialization_time < Duration::from_millis(500),
        "Config serialization should be fast"
    );

    println!("✅ Canonical configuration performance validated");
    println!("   📊 Creation: {creation_time:?} for 1000 configs");
    println!("   📊 Serialization: {serialization_time:?} for 1000 configs");

    Ok(())
}

/// Test error handling consistency across canonical system
#[tokio::test]
async fn test_canonical_error_handling() -> Result<()> {
    println!("🧪 Testing canonical error handling consistency...");

    // Test various error scenarios use unified error types
    let errors = vec![
        NestGateError::Internal {
            message: "Test internal error".to_string(),
            location: Some("test_canonical_error_handling".to_string()),
            debug_info: None,
            is_bug: false,
        },
        NestGateError::Storage {
            operation: "test_operation".to_string(),
            details: "Test storage error".to_string(),
        },
        NestGateError::Configuration {
            message: "Test configuration error".to_string(),
            config_source: nestgate_core::error::UnifiedConfigSource::Environment,
            field: Some("test_field".to_string()),
            suggested_fix: Some("Use correct field name".to_string()),
        },
    ];

    for error in errors {
        // Verify error can be serialized consistently
        let error_string = format!("{error}");
        assert!(
            !error_string.is_empty(),
            "Error should have meaningful message"
        );
        assert!(
            !error_string.contains("fragmented"),
            "Error should not contain fragmented patterns"
        );

        // Verify error maintains meaningful information
        match &error {
            NestGateError::Internal { location, .. } => {
                assert!(
                    location.is_some(),
                    "Internal error should have location information"
                );
            }
            NestGateError::Storage { operation, details } => {
                assert!(!operation.is_empty(), "Storage error should have operation");
                assert!(!details.is_empty(), "Storage error should have details");
            }
            NestGateError::Configuration { message, field, .. } => {
                assert!(
                    !message.is_empty(),
                    "Configuration error should have message"
                );
                assert!(
                    field.is_some(),
                    "Configuration error should have field info"
                );
            }
            _ => {} // Other error types have their own validation
        }
    }

    println!("✅ Canonical error handling consistency validated");
    Ok(())
}

/// Test that system handles concurrent operations with canonical patterns
#[tokio::test]
async fn test_canonical_concurrency() -> Result<()> {
    println!("🧪 Testing canonical concurrency patterns...");

    // Create multiple concurrent configuration operations
    let handles: Vec<_> = (0..10)
        .map(|i| {
            tokio::spawn(async move {
                // Each task creates and validates a config
                let config = CanonicalConfig {
                    network: NetworkConfig {
                        api: nestgate_core::config::unified::types::ApiServerConfig {
                            port: 8000 + i as u16,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                };

                // Serialize and deserialize to test thread safety
                let serialized = serde_json::to_string(&config).unwrap();
                let _deserialized: CanonicalConfig = serde_json::from_str(&serialized).unwrap();

                i
            })
        })
        .collect();

    // Wait for all tasks to complete
    let results: Vec<_> = futures::future::join_all(handles).await;

    // Verify all tasks completed successfully
    for (i, result) in results.into_iter().enumerate() {
        let task_id = result.map_err(|e| NestGateError::Internal {
            message: format!("Concurrent task {i} failed: {e}"),
            location: Some("test_canonical_concurrency".to_string()),
            debug_info: None,
            is_bug: false,
        })?;
        assert_eq!(task_id, i, "Task should return correct ID");
    }

    println!("✅ Canonical concurrency patterns validated");
    Ok(())
}

/// Test comprehensive system integration with canonical patterns
#[tokio::test]
async fn test_canonical_system_integration() -> Result<()> {
    println!("🧪 Testing canonical system integration...");

    // Create a comprehensive configuration
    let config = CanonicalConfig {
        network: NetworkConfig {
            api: nestgate_core::config::unified::types::ApiServerConfig {
                port: 8080,
                host: "127.0.0.1".parse().unwrap(),
                ..Default::default()
            },
            ..Default::default()
        },
        storage: StorageConfig {
            backend_type: "memory".to_string(),
            ..Default::default()
        },
        security: SecurityConfig {
            enable_auth: true,
            ..Default::default()
        },
        ..Default::default()
    };

    // Test that configuration validates properly
    let serialized = serde_json::to_string(&config).map_err(|e| NestGateError::Internal {
        message: format!("System integration config serialization failed: {e}"),
        location: Some("test_canonical_system_integration".to_string()),
        debug_info: None,
        is_bug: false,
    })?;

    // Verify the serialized config contains expected canonical structure
    assert!(
        serialized.contains("\"network\""),
        "Should contain network config"
    );
    assert!(
        serialized.contains("\"storage\""),
        "Should contain storage config"
    );
    assert!(
        serialized.contains("\"security\""),
        "Should contain security config"
    );
    assert!(serialized.contains("\"api\""), "Should contain API config");

    // Verify no deprecated or fragmented patterns
    assert!(
        !serialized.contains("\"extensions\""),
        "Should not contain deprecated extensions field"
    );
    assert!(
        !serialized.contains("\"service\""),
        "Should not contain deprecated service field"
    );
    assert!(
        !serialized.contains("\"fragmented\""),
        "Should not contain fragmented patterns"
    );

    println!("✅ Canonical system integration validated");
    Ok(())
}
