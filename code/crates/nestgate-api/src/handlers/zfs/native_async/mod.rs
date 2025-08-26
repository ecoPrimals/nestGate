/// This module was split from native_async_zfs.rs to maintain the 2000-line limit
/// while preserving all functionality and maintaining backward compatibility

// Sub-module declarations
pub mod traits;
pub mod implementations;

// Re-export all public types and traits for backward compatibility
pub use traits::*;
pub use implementations::*;

// Convenience re-exports for common usage patterns
pub use implementations::{ProductionZfsService, DevelopmentZfsService};

/// Convenience function to create a production ZFS service
pub fn create_production_zfs_service() -> ProductionZfsService {
    ProductionZfsService::new()
    }

/// Convenience function to create a development ZFS service
pub fn create_development_zfs_service() -> DevelopmentZfsService {
    DevelopmentZfsService::default()
    }

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::universal_zfs::types::*;

    #[tokio::test]
    async fn test_production_service_creation() {
        let service = create_production_zfs_service();
        assert_eq!(service.service_name(), "ProductionZfsService");
        assert_eq!(service.service_version(), "1.0.0");
    }

    #[tokio::test]
    async fn test_production_health_check() {
        let service = create_production_zfs_service();
        let health = service.health_check().await.unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format!("Operation failed: {:?}", e)
).into())
});
        assert!(matches!(health, HealthStatus::Healthy));
    }

    #[tokio::test]
    async fn test_production_service_availability() {
        let service = create_production_zfs_service();
        let available = service.is_available().await;
        assert!(available);
    }

    #[tokio::test]
    async fn test_production_pool_operations() {
        let service = create_production_zfs_service();
        
        // Test pool listing
        let pools = service.list_pools().await.unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format!("Operation failed: {:?}", e)
).into())
});
        assert_eq!(pools.len(), 1);
        assert_eq!(pools[0].name, "production-pool");
        
        // Test pool retrieval
        let pool = service.get_pool("production-pool").await.unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format!("Operation failed: {:?}", e)
).into())
});
        assert!(pool.is_some());
        assert_eq!(pool.unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format!("Operation failed: {:?}", e)
).into())
}).name, "production-pool");
    }

    #[tokio::test]
    async fn test_development_service_creation() {
        let service = create_development_zfs_service();
        assert_eq!(service.service_name(), "DevelopmentZfsService");
        assert_eq!(service.service_version(), "dev-1.0.0");
    }

    #[tokio::test]
    async fn test_development_pool_operations() {
        let service = create_development_zfs_service();
        
        // Test pool listing
        let pools = service.list_pools().await.unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format!("Operation failed: {:?}", e)
).into())
});
        assert_eq!(pools.len(), 1);
        assert_eq!(pools[0].name, "dev-pool");
        
        // Test pool retrieval
        let pool = service.get_pool("dev-pool").await.unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format!("Operation failed: {:?}", e)
).into())
});
        assert!(pool.is_some());
        assert_eq!(pool.unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format!("Operation failed: {:?}", e)
).into())
}).name, "dev-pool");
    }

    #[tokio::test]
    async fn test_dataset_operations() {
        let service = create_production_zfs_service();
        
        // Test dataset listing
        let datasets = service.list_datasets(Some("production-pool")).await.unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format!("Operation failed: {:?}", e)
).into())
});
        assert_eq!(datasets.len(), 1);
        assert_eq!(datasets[0].name, "production-pool/data");
        
        // Test dataset retrieval
        let dataset = service.get_dataset("production-pool/data").await.unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format!("Operation failed: {:?}", e)
).into())
});
        assert!(dataset.is_some());
        assert_eq!(dataset.unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format!("Operation failed: {:?}", e)
).into())
}).name, "production-pool/data");
    }

    #[tokio::test]
    async fn test_snapshot_operations() {
        let service = create_production_zfs_service();
        
        // Test snapshot listing
        let snapshots = service.list_snapshots(Some("production-pool/data")).await.unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format!("Operation failed: {:?}", e)
).into())
});
        assert_eq!(snapshots.len(), 1);
        assert!(snapshots[0].name.contains("production-pool/data@backup-"));
        
        // Test snapshot creation
        let snapshot_config = SnapshotConfig {
            name: "production-pool/data@test-snapshot".to_string(),
            dataset_name: "production-pool/data".to_string(),
            recursive: false,
            properties: std::collections::HashMap::new(),
        };
        
        let snapshot = service.create_snapshot(&snapshot_config).await.unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format!("Operation failed: {:?}", e)
).into())
});
        assert_eq!(snapshot.name, "production-pool/data@test-snapshot");
        assert_eq!(snapshot.dataset_name, "production-pool/data");
    }

    #[tokio::test]
    async fn test_bulk_operations() {
        let service = create_production_zfs_service();
        
        // Test bulk snapshot creation
        let configs = vec![
            SnapshotConfig {
                name: "production-pool/data@bulk-1".to_string(),
                dataset_name: "production-pool/data".to_string(),
                recursive: false,
                properties: std::collections::HashMap::new(),
            },
            SnapshotConfig {
                name: "production-pool/data@bulk-2".to_string(),
                dataset_name: "production-pool/data".to_string(),
                recursive: false,
                properties: std::collections::HashMap::new(),
            },
        ];
        
        let snapshots = service.bulk_create_snapshots(&configs).await.unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format!("Operation failed: {:?}", e)
).into())
});
        assert_eq!(snapshots.len(), 2);
        assert_eq!(snapshots[0].name, "production-pool/data@bulk-1");
        assert_eq!(snapshots[1].name, "production-pool/data@bulk-2");
    }
} 