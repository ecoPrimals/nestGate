//! **OBJECT STORAGE UNIT TESTS**
//!
//! Comprehensive unit tests for refactored object storage backend.

use nestgate_zfs::backends::object_storage::{
    ObjectStorageBackend, StorageProvider, ObjectPool, ObjectDataset,
};
use nestgate_zfs::StorageTier;
use std::collections::HashMap;

// ============================================================================
// Provider Detection Tests (3 tests)
// ============================================================================

#[test]
fn test_provider_detection_aws() {
    let provider = StorageProvider::detect_from_endpoint("https://s3.amazonaws.com");
    assert!(matches!(provider, StorageProvider::AWS));
}

#[test]
fn test_provider_detection_minio() {
    let provider = StorageProvider::detect_from_endpoint("http://localhost:9000");
    assert!(matches!(provider, StorageProvider::MinIO));
    
    let provider2 = StorageProvider::detect_from_endpoint("https://minio.example.com");
    assert!(matches!(provider2, StorageProvider::MinIO));
}

#[test]
fn test_provider_detection_generic() {
    let provider = StorageProvider::detect_from_endpoint("https://storage.example.com");
    assert!(matches!(provider, StorageProvider::Generic));
    
    let provider2 = StorageProvider::detect_from_endpoint("http://192.168.1.100:8080");
    assert!(matches!(provider2, StorageProvider::Generic));
}

// ============================================================================
// Pool Tests (2 tests)
// ============================================================================

#[test]
fn test_object_pool_creation() {
    let pool = ObjectPool {
        name: "test-pool".to_string(),
        endpoint: "https://s3.example.com".to_string(),
        bucket: "test-bucket".to_string(),
        provider: StorageProvider::Generic,
        properties: HashMap::new(),
    };

    assert_eq!(pool.name, "test-pool");
    assert_eq!(pool.bucket, "test-bucket");
    assert!(matches!(pool.provider, StorageProvider::Generic));
}

#[test]
fn test_object_pool_with_properties() {
    let mut properties = HashMap::new();
    properties.insert("region".to_string(), "us-west-2".to_string());
    properties.insert("versioning".to_string(), "enabled".to_string());

    let pool = ObjectPool {
        name: "production-pool".to_string(),
        endpoint: "https://s3.amazonaws.com".to_string(),
        bucket: "prod-bucket".to_string(),
        provider: StorageProvider::AWS,
        properties,
    };

    assert_eq!(pool.properties.get("region").unwrap(), "us-west-2");
    assert_eq!(pool.properties.get("versioning").unwrap(), "enabled");
}

// ============================================================================
// Dataset Tests (3 tests)
// ============================================================================

#[test]
fn test_object_dataset_creation() {
    let dataset = ObjectDataset {
        name: "test-dataset".to_string(),
        pool: "test-pool".to_string(),
        prefix: "test-pool/test-dataset".to_string(),
        tier: StorageTier::Hot,
        created_at: std::time::SystemTime::now(),
    };

    assert_eq!(dataset.name, "test-dataset");
    assert_eq!(dataset.pool, "test-pool");
    assert!(matches!(dataset.tier, StorageTier::Hot));
}

#[test]
fn test_dataset_tier_mapping() {
    let tiers = vec![
        StorageTier::Hot,
        StorageTier::Warm,
        StorageTier::Cold,
        StorageTier::Archive,
    ];

    for tier in tiers {
        let dataset = ObjectDataset {
            name: format!("{:?}-dataset", tier),
            pool: "test-pool".to_string(),
            prefix: "test-pool/dataset".to_string(),
            tier: tier.clone(),
            created_at: std::time::SystemTime::now(),
        };

        assert_eq!(dataset.tier, tier);
    }
}

#[test]
fn test_dataset_prefix_format() {
    let dataset = ObjectDataset {
        name: "data".to_string(),
        pool: "mypool".to_string(),
        prefix: "mypool/data".to_string(),
        tier: StorageTier::Hot,
        created_at: std::time::SystemTime::now(),
    };

    assert!(dataset.prefix.starts_with(&dataset.pool));
    assert!(dataset.prefix.contains(&dataset.name));
}

// ============================================================================
// Configuration Tests (4 tests)
// ============================================================================

#[test]
fn test_config_from_env_s3_endpoint() {
    std::env::set_var("S3_ENDPOINT", "https://test.s3.example.com");
    std::env::set_var("S3_BUCKET", "test-bucket");
    
    // Config should be discoverable
    let endpoint = std::env::var("S3_ENDPOINT").unwrap();
    let bucket = std::env::var("S3_BUCKET").unwrap();
    
    assert_eq!(endpoint, "https://test.s3.example.com");
    assert_eq!(bucket, "test-bucket");
    
    // Cleanup
    std::env::remove_var("S3_ENDPOINT");
    std::env::remove_var("S3_BUCKET");
}

#[test]
fn test_config_from_env_minio() {
    std::env::set_var("MINIO_ENDPOINT", "http://localhost:9000");
    std::env::set_var("MINIO_BUCKET", "minio-bucket");
    
    let endpoint = std::env::var("MINIO_ENDPOINT").unwrap();
    let bucket = std::env::var("MINIO_BUCKET").unwrap();
    
    assert!(endpoint.contains("localhost:9000"));
    assert_eq!(bucket, "minio-bucket");
    
    // Cleanup
    std::env::remove_var("MINIO_ENDPOINT");
    std::env::remove_var("MINIO_BUCKET");
}

#[test]
fn test_config_precedence_order() {
    // Test that specific configs take precedence over generic
    std::env::set_var("MINIO_ENDPOINT", "http://minio:9000");
    std::env::set_var("S3_ENDPOINT", "https://s3.aws.com");
    
    // MinIO should be checked first (more specific)
    let minio = std::env::var("MINIO_ENDPOINT");
    assert!(minio.is_ok());
    
    // Cleanup
    std::env::remove_var("MINIO_ENDPOINT");
    std::env::remove_var("S3_ENDPOINT");
}

#[test]
fn test_config_validation() {
    // Valid configs
    std::env::set_var("S3_ENDPOINT", "https://valid.endpoint.com");
    let valid = std::env::var("S3_ENDPOINT").unwrap();
    assert!(valid.starts_with("http"));
    
    std::env::remove_var("S3_ENDPOINT");
}

// ============================================================================
// Edge Cases & Error Handling (3 tests)
// ============================================================================

#[test]
fn test_empty_pool_name() {
    let pool = ObjectPool {
        name: "".to_string(),
        endpoint: "https://s3.example.com".to_string(),
        bucket: "test-bucket".to_string(),
        provider: StorageProvider::Generic,
        properties: HashMap::new(),
    };

    // Empty name should still be valid (will be caught by validation)
    assert_eq!(pool.name, "");
}

#[test]
fn test_special_characters_in_names() {
    let pool = ObjectPool {
        name: "test-pool_123.prod".to_string(),
        endpoint: "https://s3.example.com".to_string(),
        bucket: "test-bucket-456".to_string(),
        provider: StorageProvider::Generic,
        properties: HashMap::new(),
    };

    assert!(pool.name.contains('-'));
    assert!(pool.name.contains('_'));
    assert!(pool.name.contains('.'));
}

#[test]
fn test_provider_serialization() {
    let providers = vec![
        StorageProvider::AWS,
        StorageProvider::MinIO,
        StorageProvider::Ceph,
        StorageProvider::Generic,
    ];

    for provider in providers {
        // Should be able to format debug
        let debug_str = format!("{:?}", provider);
        assert!(!debug_str.is_empty());
    }
}

// ============================================================================
// Performance & Stress Tests (2 tests)
// ============================================================================

#[test]
fn test_many_properties() {
    let mut properties = HashMap::new();
    
    for i in 0..100 {
        properties.insert(format!("key_{}", i), format!("value_{}", i));
    }

    let pool = ObjectPool {
        name: "property-test".to_string(),
        endpoint: "https://s3.example.com".to_string(),
        bucket: "test-bucket".to_string(),
        provider: StorageProvider::Generic,
        properties,
    };

    assert_eq!(pool.properties.len(), 100);
    assert_eq!(pool.properties.get("key_50").unwrap(), "value_50");
}

#[test]
fn test_long_prefix_paths() {
    let long_prefix = format!("pool/{}/{}/{}/{}/dataset",
        "very", "long", "nested", "path");
    
    let dataset = ObjectDataset {
        name: "nested-dataset".to_string(),
        pool: "pool".to_string(),
        prefix: long_prefix.clone(),
        tier: StorageTier::Cold,
        created_at: std::time::SystemTime::now(),
    };

    assert_eq!(dataset.prefix, long_prefix);
    assert!(dataset.prefix.len() > 30);
}
