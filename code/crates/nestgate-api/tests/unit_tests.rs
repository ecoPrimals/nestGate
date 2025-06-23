//! Unit tests for NestGate API components
//!
//! This test suite validates individual API components, handlers, and utilities
//! in isolation using mocks and test doubles.

use std::collections::HashMap;
use nestgate_core::StorageTier;
use nestgate_api::handlers::zfs::*;

/// Test the API response structure
#[test]
fn test_api_response_success() {
    let response = ApiResponse::success("test data".to_string());
    
    assert_eq!(response.success, true);
    assert_eq!(response.data, Some("test data".to_string()));
    assert!(response.error.is_none());
    assert!(!response.timestamp.to_string().is_empty());
}

#[test]
fn test_api_response_error() {
    let response = ApiResponse::<()>::error_empty("Test error message".to_string());
    
    assert_eq!(response.success, false);
    assert!(response.data.is_none());
    assert_eq!(response.error, Some("Test error message".to_string()));
    assert!(!response.timestamp.to_string().is_empty());
}

/// Test request model validation
#[test]
fn test_create_pool_request_serialization() {
    let request = CreatePoolRequest {
        name: "test_pool".to_string(),
        devices: vec!["/dev/sda".to_string(), "/dev/sdb".to_string()],
        config: Some(PoolConfig {
            raid_level: Some("mirror".to_string()),
            compression: Some("lz4".to_string()),
            dedup: Some(false),
            encryption: Some(true),
        }),
    };

    let json_str = serde_json::to_string(&request).expect("Failed to serialize");
    let parsed: CreatePoolRequest = serde_json::from_str(&json_str).expect("Failed to deserialize");

    assert_eq!(parsed.name, "test_pool");
    assert_eq!(parsed.devices.len(), 2);
    assert!(parsed.config.is_some());
    
    let config = parsed.config.unwrap();
    assert_eq!(config.raid_level, Some("mirror".to_string()));
    assert_eq!(config.compression, Some("lz4".to_string()));
    assert_eq!(config.dedup, Some(false));
    assert_eq!(config.encryption, Some(true));
}

#[test]
fn test_create_dataset_request_serialization() {
    let request = CreateDatasetRequest {
        name: "test_dataset".to_string(),
        parent: "test_pool".to_string(),
        tier: StorageTier::Warm,
        properties: Some({
            let mut props = HashMap::new();
            props.insert("compression".to_string(), "lz4".to_string());
            props.insert("recordsize".to_string(), "128K".to_string());
            props
        }),
    };

    let json_str = serde_json::to_string(&request).expect("Failed to serialize");
    let parsed: CreateDatasetRequest = serde_json::from_str(&json_str).expect("Failed to deserialize");

    assert_eq!(parsed.name, "test_dataset");
    assert_eq!(parsed.parent, "test_pool");
    assert_eq!(parsed.tier, StorageTier::Warm);
    assert!(parsed.properties.is_some());
    
    let props = parsed.properties.unwrap();
    assert_eq!(props.get("compression"), Some(&"lz4".to_string()));
    assert_eq!(props.get("recordsize"), Some(&"128K".to_string()));
}

#[test]
fn test_create_snapshot_request_serialization() {
    let request = CreateSnapshotRequest {
        name: "test_snapshot".to_string(),
        dataset: "test_dataset".to_string(),
        recursive: Some(true),
        properties: Some({
            let mut props = HashMap::new();
            props.insert("comment".to_string(), "Test snapshot".to_string());
            props
        }),
    };

    let json_str = serde_json::to_string(&request).expect("Failed to serialize");
    let parsed: CreateSnapshotRequest = serde_json::from_str(&json_str).expect("Failed to deserialize");

    assert_eq!(parsed.name, "test_snapshot");
    assert_eq!(parsed.dataset, "test_dataset");
    assert_eq!(parsed.recursive, Some(true));
    assert!(parsed.properties.is_some());
}

#[test]
fn test_tier_migration_request_serialization() {
    let request = TierMigrationRequest {
        dataset_path: "/test/dataset".to_string(),
        source_tier: StorageTier::Hot,
        target_tier: StorageTier::Cold,
        priority: Some(5),
        force: Some(false),
    };

    let json_str = serde_json::to_string(&request).expect("Failed to serialize");
    let parsed: TierMigrationRequest = serde_json::from_str(&json_str).expect("Failed to deserialize");

    assert_eq!(parsed.dataset_path, "/test/dataset");
    assert_eq!(parsed.source_tier, StorageTier::Hot);
    assert_eq!(parsed.target_tier, StorageTier::Cold);
    assert_eq!(parsed.priority, Some(5));
    assert_eq!(parsed.force, Some(false));
}

#[test]
fn test_list_query_serialization() {
    let query = ListQuery {
        limit: Some(10),
        skip: Some(5),
        status: Some("healthy".to_string()),
        tier: Some(StorageTier::Warm),
    };

    let json_str = serde_json::to_string(&query).expect("Failed to serialize");
    let parsed: ListQuery = serde_json::from_str(&json_str).expect("Failed to deserialize");

    assert_eq!(parsed.limit, Some(10));
    assert_eq!(parsed.skip, Some(5));
    assert_eq!(parsed.status, Some("healthy".to_string()));
    assert_eq!(parsed.tier, Some(StorageTier::Warm));
}

#[test]
fn test_tier_prediction_request_serialization() {
    let request = TierPredictionRequest {
        file_path: "/test/file.txt".to_string(),
    };

    let json_str = serde_json::to_string(&request).expect("Failed to serialize");
    let parsed: TierPredictionRequest = serde_json::from_str(&json_str).expect("Failed to deserialize");

    assert_eq!(parsed.file_path, "/test/file.txt");
}

/// Test JSON validation and error cases
#[test]
fn test_invalid_json_handling() {
    // Test invalid tier value
    let invalid_json = r#"{"name": "test", "parent": "pool", "tier": "InvalidTier"}"#;
    let result: Result<CreateDatasetRequest, _> = serde_json::from_str(invalid_json);
    assert!(result.is_err());
    
    // Test missing required fields
    let incomplete_json = r#"{"name": "test"}"#;
    let result: Result<CreateDatasetRequest, _> = serde_json::from_str(incomplete_json);
    assert!(result.is_err());
}

#[test]
fn test_storage_tier_serialization() {
    // Test all storage tier variants
    let tiers = vec![
        StorageTier::Hot,
        StorageTier::Warm,
        StorageTier::Cold,
        StorageTier::Cache,
    ];

    for tier in tiers {
        let json_str = serde_json::to_string(&tier).expect("Failed to serialize tier");
        let parsed: StorageTier = serde_json::from_str(&json_str).expect("Failed to deserialize tier");
        assert_eq!(parsed, tier);
    }
}

/// Test optional field handling
#[test]
fn test_optional_fields_handling() {
    // Test CreatePoolRequest with minimal fields
    let minimal_json = r#"{"name": "test_pool", "devices": ["/dev/sda"]}"#;
    let parsed: CreatePoolRequest = serde_json::from_str(minimal_json).expect("Failed to parse minimal request");
    
    assert_eq!(parsed.name, "test_pool");
    assert_eq!(parsed.devices.len(), 1);
    assert!(parsed.config.is_none());
    
    // Test CreateDatasetRequest with minimal fields
    let minimal_json = r#"{"name": "test_dataset", "parent": "test_pool", "tier": "Warm"}"#;
    let parsed: CreateDatasetRequest = serde_json::from_str(minimal_json).expect("Failed to parse minimal request");
    
    assert_eq!(parsed.name, "test_dataset");
    assert_eq!(parsed.parent, "test_pool");
    assert_eq!(parsed.tier, StorageTier::Warm);
    assert!(parsed.properties.is_none());
}

/// Test API response timestamp format
#[test]
fn test_api_response_timestamp_format() {
    let response = ApiResponse::success("test".to_string());
    
    // Timestamp should be a valid DateTime
    let timestamp_str = response.timestamp.to_rfc3339();
    assert!(timestamp_str.contains('T'));
    assert!(timestamp_str.contains('Z') || timestamp_str.contains('+'));
    
    // Should be parseable as DateTime
    let _parsed_time: chrono::DateTime<chrono::Utc> = timestamp_str.parse()
        .expect("Timestamp should be valid ISO 8601");
}

/// Test edge cases and boundary conditions
#[test]
fn test_edge_cases() {
    // Test empty device list
    let request = CreatePoolRequest {
        name: "test_pool".to_string(),
        devices: vec![],
        config: None,
    };
    
    let json_str = serde_json::to_string(&request).expect("Failed to serialize");
    let parsed: CreatePoolRequest = serde_json::from_str(&json_str).expect("Failed to deserialize");
    assert!(parsed.devices.is_empty());
    
    // Test very long names
    let long_name = "a".repeat(1000);
    let request = CreatePoolRequest {
        name: long_name.clone(),
        devices: vec!["/dev/sda".to_string()],
        config: None,
    };
    
    let json_str = serde_json::to_string(&request).expect("Failed to serialize");
    let parsed: CreatePoolRequest = serde_json::from_str(&json_str).expect("Failed to deserialize");
    assert_eq!(parsed.name, long_name);
}

/// Test data structure consistency
#[test]
fn test_data_structure_consistency() {
    // Test that all request structures can be round-trip serialized
    let pool_request = CreatePoolRequest {
        name: "test".to_string(),
        devices: vec!["/dev/sda".to_string()],
        config: Some(PoolConfig {
            raid_level: Some("mirror".to_string()),
            compression: Some("lz4".to_string()),
            dedup: Some(false),
            encryption: Some(false),
        }),
    };
    
    let json = serde_json::to_string(&pool_request).unwrap();
    let parsed: CreatePoolRequest = serde_json::from_str(&json).unwrap();
    let json2 = serde_json::to_string(&parsed).unwrap();
    
    // Should be identical after round-trip
    assert_eq!(json, json2);
}

/// Test API state structure
#[test]
fn test_zfs_api_state_structure() {
    // This test validates that ZfsApiState can be constructed
    // We can't easily test it without a real ZfsManager, but we can
    // verify the structure exists and has the expected fields
    
    // The struct should be cloneable and debuggable
    use std::fmt::Debug;
    
    fn assert_clone<T: Clone>() {}
    fn assert_debug<T: Debug>() {}
    
    assert_clone::<ZfsApiState>();
    assert_debug::<ZfsApiState>();
}

/// Performance and memory tests
#[test]
fn test_serialization_performance() {
    // Test that serialization of large structures is reasonable
    let large_request = CreateDatasetRequest {
        name: "test_dataset".to_string(),
        parent: "test_pool".to_string(),
        tier: StorageTier::Warm,
        properties: Some({
            let mut props = HashMap::new();
            for i in 0..1000 {
                props.insert(format!("property_{}", i), format!("value_{}", i));
            }
            props
        }),
    };
    
    let start = std::time::Instant::now();
    let json_str = serde_json::to_string(&large_request).expect("Failed to serialize");
    let serialize_time = start.elapsed();
    
    let start = std::time::Instant::now();
    let _parsed: CreateDatasetRequest = serde_json::from_str(&json_str).expect("Failed to deserialize");
    let deserialize_time = start.elapsed();
    
    // Serialization should be reasonably fast (less than 10ms for 1000 properties)
    assert!(serialize_time.as_millis() < 10, "Serialization took too long: {:?}", serialize_time);
    assert!(deserialize_time.as_millis() < 10, "Deserialization took too long: {:?}", deserialize_time);
} 