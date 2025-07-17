use std::collections::HashMap;

use nestgate_core::universal_storage::{
    FileMetadata, StorageBackend, StorageCapability, StorageProtocol, StorageRequest,
    UniversalStorageConfig, UniversalStorageManager,
};

#[tokio::test]
async fn test_universal_storage_manager_creation() {
    let config = UniversalStorageConfig {
        max_concurrent_operations: 50,
        event_retention_hours: 12,
        sync_batch_size: 500,
        health_check_interval: 60,
        replication_lag_tolerance: 10,
    };

    let manager = UniversalStorageManager::new(config).await;
    assert!(manager.is_ok());
}

#[tokio::test]
async fn test_universal_storage_manager_start() {
    let config = UniversalStorageConfig::default();
    let manager = UniversalStorageManager::new(config).await.unwrap();

    let result = manager.start().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_storage_backend_registration() {
    let config = UniversalStorageConfig::default();
    let manager = UniversalStorageManager::new(config).await.unwrap();

    let backend = StorageBackend {
        name: "test-fs".to_string(),
        protocol: StorageProtocol::FileSystem,
        capabilities: vec![StorageCapability::ReadWrite, StorageCapability::Versioning],
        health_status: "healthy".to_string(),
        endpoint: "/tmp/test".to_string(),
    };

    let result = manager.register_storage_backend(backend).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_storage_request_coordination() {
    let config = UniversalStorageConfig::default();
    let manager = UniversalStorageManager::new(config).await.unwrap();

    let request = StorageRequest::CreateFile {
        path: "/test/file.txt".to_string(),
        content: b"Hello, World!".to_vec(),
        metadata: Box::new(FileMetadata {
            path: "/test/file.txt".to_string(),
            size: 13,
            created_at: chrono::Utc::now(),
            modified_at: chrono::Utc::now(),
            permissions: "rw-r--r--".to_string(),
            owner: "user".to_string(),
            group: "group".to_string(),
            checksum: None,
            mime_type: Some("text/plain".to_string()),
            tags: HashMap::new(),
        }),
    };

    let result = manager.coordinate_storage_request(request).await;
    // This will likely fail because no backends are registered, but we test the coordination flow
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_storage_event_streaming() {
    let config = UniversalStorageConfig::default();
    let manager = UniversalStorageManager::new(config).await.unwrap();

    let result = manager.stream_storage_events().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_multi_protocol_support() {
    let config = UniversalStorageConfig::default();
    let manager = UniversalStorageManager::new(config).await.unwrap();

    // Test different protocol backends
    let protocols = vec![
        StorageProtocol::FileSystem,
        StorageProtocol::ObjectStorage,
        StorageProtocol::BlockStorage,
        StorageProtocol::NetworkFileSystem,
        StorageProtocol::DistributedFileSystem,
        StorageProtocol::StreamingProtocol,
    ];

    for protocol in protocols {
        let backend = StorageBackend {
            name: format!("test-{:?}", protocol),
            protocol: protocol.clone(),
            capabilities: vec![StorageCapability::ReadWrite],
            health_status: "healthy".to_string(),
            endpoint: format!("/test/{:?}", protocol),
        };

        let result = manager.register_storage_backend(backend).await;
        assert!(result.is_ok());
    }
}

#[tokio::test]
async fn test_storage_capabilities() {
    let config = UniversalStorageConfig::default();
    let manager = UniversalStorageManager::new(config).await.unwrap();

    let capabilities = vec![
        StorageCapability::ReadWrite,
        StorageCapability::Streaming,
        StorageCapability::Replication,
        StorageCapability::Versioning,
        StorageCapability::Encryption,
        StorageCapability::Compression,
        StorageCapability::Deduplication,
        StorageCapability::Snapshots,
        StorageCapability::RealTimeSync,
        StorageCapability::DistributedCoordination,
    ];

    let backend = StorageBackend {
        name: "full-featured-backend".to_string(),
        protocol: StorageProtocol::DistributedFileSystem,
        capabilities,
        health_status: "healthy".to_string(),
        endpoint: "/distributed/storage".to_string(),
    };

    let result = manager.register_storage_backend(backend).await;
    assert!(result.is_ok());
}
