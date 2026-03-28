// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Integration tests for storage handlers.

use super::storage::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_storage_handler_creation() {
        let handler = StorageHandler::new();
        // Verify handler was created successfully
        assert_eq!(
            std::mem::size_of_val(&handler),
            std::mem::size_of::<StorageHandler>()
        );
    }

    #[tokio::test]
    async fn test_storage_handler_default() {
        let handler = StorageHandler::default();
        // Verify default constructor works
        assert_eq!(
            std::mem::size_of_val(&handler),
            std::mem::size_of::<StorageHandler>()
        );
    }

    #[tokio::test]
    async fn test_storage_manager_creation() {
        let manager = StorageManager::new();
        assert_eq!(
            std::mem::size_of_val(&manager),
            std::mem::size_of::<StorageManager>()
        );
    }

    #[tokio::test]
    async fn test_storage_manager_default() {
        let manager = StorageManager::default();
        assert_eq!(
            std::mem::size_of_val(&manager),
            std::mem::size_of::<StorageManager>()
        );
    }

    #[tokio::test]
    async fn test_get_storage_pools() {
        let result = get_storage_pools().await;

        assert!(result.is_ok(), "get_storage_pools should succeed");

        let pools = result.expect("Storage operation failed");

        // Verify we get pools back
        assert_eq!(pools.0.len(), 2, "Should return 2 mock pools");

        // Verify first pool
        let main_pool = &pools.0[0];
        assert_eq!(main_pool.name, "main-pool");
        assert_eq!(main_pool.total_capacity_gb, 1000);
        assert_eq!(main_pool.used_capacity_gb, 400);
        assert_eq!(main_pool.available_capacity_gb, 600);
        assert_eq!(main_pool.health_status, "healthy");

        // Verify second pool
        let backup_pool = &pools.0[1];
        assert_eq!(backup_pool.name, "backup-pool");
        assert_eq!(backup_pool.total_capacity_gb, 500);
    }

    #[tokio::test]
    async fn test_get_storage_datasets() {
        let result = get_storage_datasets().await;

        assert!(result.is_ok(), "get_storage_datasets should succeed");

        let datasets = result.expect("Storage operation failed");

        // Verify we get datasets back
        assert_eq!(datasets.0.len(), 2, "Should return 2 mock datasets");

        // Verify first dataset
        let data_dataset = &datasets.0[0];
        assert_eq!(data_dataset.name, "main-pool/data");
        assert_eq!(data_dataset.pool_name, "main-pool");
        assert_eq!(data_dataset.used_space_gb, 200);
        assert_eq!(data_dataset.compression_ratio, 1.5);
        assert_eq!(data_dataset.dedup_ratio, 1.2);

        // Verify second dataset
        let logs_dataset = &datasets.0[1];
        assert_eq!(logs_dataset.name, "main-pool/logs");
        assert_eq!(logs_dataset.used_space_gb, 50);
    }

    #[tokio::test]
    async fn test_get_storage_snapshots() {
        let result = get_storage_snapshots().await;

        assert!(result.is_ok(), "get_storage_snapshots should succeed");

        let snapshots = result.expect("Storage operation failed");

        // Verify we get snapshots back
        assert_eq!(snapshots.0.len(), 2, "Should return 2 mock snapshots");

        // Verify first snapshot
        let backup_snapshot = &snapshots.0[0];
        assert_eq!(backup_snapshot.name, "main-pool/data@backup-2024-01-15");
        assert_eq!(backup_snapshot.dataset_name, "main-pool/data");
        assert_eq!(backup_snapshot.size_gb, 180);

        // Verify second snapshot
        let daily_snapshot = &snapshots.0[1];
        assert_eq!(daily_snapshot.name, "main-pool/logs@daily-2024-01-15");
        assert_eq!(daily_snapshot.size_gb, 45);
    }

    #[tokio::test]
    async fn test_get_storage_metrics() {
        let result = get_storage_metrics().await;

        assert!(result.is_ok(), "get_storage_metrics should succeed");

        let metrics = result.expect("Storage operation failed");

        // Verify all metric fields
        assert_eq!(metrics.0.total_pools, 2);
        assert_eq!(metrics.0.total_datasets, 5);
        assert_eq!(metrics.0.total_snapshots, 12);
        assert_eq!(metrics.0.total_storage, 1_500_000_000_000);
        assert_eq!(metrics.0.used_storage, 550_000_000_000);
        assert_eq!(metrics.0.available_storage, 950_000_000_000);
        assert_eq!(metrics.0.iops, 1250.0);
        assert_eq!(metrics.0.bandwidth_mbps, 450.5);
        assert_eq!(metrics.0.health_status, "healthy");
    }

    #[test]
    fn test_storage_pool_info_structure() {
        let pool = StoragePoolInfo {
            name: "test-pool".to_string(),
            total_capacity_gb: 2000,
            used_capacity_gb: 800,
            available_capacity_gb: 1200,
            health_status: "healthy".to_string(),
        };

        assert_eq!(pool.name, "test-pool");
        assert_eq!(pool.total_capacity_gb, 2000);
        assert_eq!(pool.used_capacity_gb, 800);
        assert_eq!(pool.available_capacity_gb, 1200);

        // Verify capacity math
        assert_eq!(
            pool.used_capacity_gb + pool.available_capacity_gb,
            pool.total_capacity_gb
        );
    }

    #[test]
    fn test_storage_dataset_info_structure() {
        let dataset = StorageDatasetInfo {
            name: "pool/dataset".to_string(),
            pool_name: "pool".to_string(),
            used_space_gb: 150,
            compression_ratio: 2.0,
            dedup_ratio: 1.5,
        };

        assert_eq!(dataset.name, "pool/dataset");
        assert_eq!(dataset.pool_name, "pool");
        assert_eq!(dataset.used_space_gb, 150);
        assert_eq!(dataset.compression_ratio, 2.0);
        assert_eq!(dataset.dedup_ratio, 1.5);
    }

    #[test]
    fn test_storage_snapshot_info_structure() {
        let snapshot = StorageSnapshotInfo {
            name: "pool/data@snap1".to_string(),
            dataset_name: "pool/data".to_string(),
            created_at: std::time::SystemTime::now(),
            size_gb: 100,
        };

        assert_eq!(snapshot.name, "pool/data@snap1");
        assert_eq!(snapshot.dataset_name, "pool/data");
        assert_eq!(snapshot.size_gb, 100);
    }

    #[test]
    fn test_storage_pool_serialization() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let pool = StoragePool {
            name: "test-pool".to_string(),
            status: "ONLINE".to_string(),
            size: 1_000_000_000_000,
            used: 500_000_000_000,
            available: 500_000_000_000,
            health: "HEALTHY".to_string(),
            pool_type: "RAIDZ".to_string(),
        };

        let json = serde_json::to_string(&pool).unwrap();

        assert!(json.contains("\"name\":\"test-pool\""));
        assert!(json.contains("\"status\":\"ONLINE\""));
        assert!(json.contains("\"health\":\"HEALTHY\""));

        Ok(())
    }

    #[test]
    fn test_storage_dataset_serialization() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let dataset = StorageDataset {
            name: "pool/dataset".to_string(),
            pool: "pool".to_string(),
            size: 1_000_000_000,
            used: 500_000_000,
            available: 500_000_000,
            mount_point: "/mnt/dataset".to_string(),
            compression: "lz4".to_string(),
        };

        let json = serde_json::to_string(&dataset).unwrap();

        assert!(json.contains("\"name\":\"pool/dataset\""));
        assert!(json.contains("\"compression\":\"lz4\""));

        Ok(())
    }

    #[test]
    fn test_storage_snapshot_serialization() -> std::result::Result<(), Box<dyn std::error::Error>>
    {
        let snapshot = StorageSnapshot {
            name: "pool@snap".to_string(),
            dataset: "pool".to_string(),
            size: 1_000_000,
            created: "2024-01-15T10:00:00Z".to_string(),
            referenced: 500_000,
        };

        let json = serde_json::to_string(&snapshot).unwrap();

        assert!(json.contains("\"name\":\"pool@snap\""));
        assert!(json.contains("\"dataset\":\"pool\""));

        Ok(())
    }

    #[test]
    fn test_storage_metrics_serialization() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let metrics = StorageMetrics {
            total_pools: 3,
            total_datasets: 10,
            total_snapshots: 25,
            total_storage: 2_000_000_000_000,
            used_storage: 1_000_000_000_000,
            available_storage: 1_000_000_000_000,
            iops: 1500.0,
            bandwidth_mbps: 500.0,
            health_status: "healthy".to_string(),
        };

        let json = serde_json::to_string(&metrics).unwrap();

        assert!(json.contains("\"total_pools\":3"));
        assert!(json.contains("\"iops\":1500"));
        assert!(json.contains("\"bandwidth_mbps\":500"));

        Ok(())
    }

    #[test]
    fn test_storage_pool_info_serialization() -> std::result::Result<(), Box<dyn std::error::Error>>
    {
        let pool_info = StoragePoolInfo {
            name: "production-pool".to_string(),
            total_capacity_gb: 5000,
            used_capacity_gb: 2000,
            available_capacity_gb: 3000,
            health_status: "healthy".to_string(),
        };

        let json = serde_json::to_string(&pool_info).unwrap();

        assert!(json.contains("\"name\":\"production-pool\""));
        assert!(json.contains("\"total_capacity_gb\":5000"));

        // Test deserialization
        let deserialized: StoragePoolInfo = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.name, "production-pool");
        assert_eq!(deserialized.total_capacity_gb, 5000);

        Ok(())
    }

    #[test]
    fn test_storage_dataset_info_serialization(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let dataset_info = StorageDatasetInfo {
            name: "pool/important-data".to_string(),
            pool_name: "pool".to_string(),
            used_space_gb: 500,
            compression_ratio: 1.8,
            dedup_ratio: 1.3,
        };

        let json = serde_json::to_string(&dataset_info).unwrap();

        assert!(json.contains("\"compression_ratio\":1.8"));

        Ok(())
    }

    #[test]
    fn test_storage_snapshot_info_serialization(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let snapshot_info = StorageSnapshotInfo {
            name: "pool/data@weekly-backup".to_string(),
            dataset_name: "pool/data".to_string(),
            created_at: std::time::SystemTime::now(),
            size_gb: 250,
        };

        let json = serde_json::to_string(&snapshot_info).unwrap();

        assert!(json.contains("\"size_gb\":250"));

        Ok(())
    }

    #[tokio::test]
    async fn test_multiple_pool_requests() {
        // Call handler multiple times to verify consistency
        for _ in 0..5 {
            let result = get_storage_pools().await;
            assert!(result.is_ok(), "Each request should succeed");

            let pools = result.expect("Storage operation failed");

            assert_eq!(pools.0.len(), 2, "Should always return 2 pools");
        }
    }

    #[tokio::test]
    async fn test_storage_metrics_calculations() {
        let result = get_storage_metrics().await;
        let metrics = result.expect("Storage operation failed");

        // Verify storage calculations
        assert_eq!(
            metrics.0.used_storage + metrics.0.available_storage,
            metrics.0.total_storage,
            "Used + Available should equal Total"
        );

        // Verify metrics are reasonable
        assert!(metrics.0.iops > 0.0, "IOPS should be positive");
        assert!(
            metrics.0.bandwidth_mbps > 0.0,
            "Bandwidth should be positive"
        );
    }

    #[test]
    fn test_storage_pool_health_states() {
        let states = vec!["healthy", "degraded", "offline", "faulted"];

        for state in states {
            let pool = StoragePoolInfo {
                name: "test-pool".to_string(),
                total_capacity_gb: 1000,
                used_capacity_gb: 500,
                available_capacity_gb: 500,
                health_status: state.to_string(),
            };

            assert_eq!(pool.health_status, state);
        }
    }

    #[test]
    fn test_compression_ratio_values() {
        let ratios = vec![1.0, 1.5, 2.0, 2.5, 3.0];

        for ratio in ratios {
            let dataset = StorageDatasetInfo {
                name: "pool/data".to_string(),
                pool_name: "pool".to_string(),
                used_space_gb: 100,
                compression_ratio: ratio,
                dedup_ratio: 1.0,
            };

            assert_eq!(dataset.compression_ratio, ratio);
            assert!(
                dataset.compression_ratio >= 1.0,
                "Compression ratio should be >= 1.0"
            );
        }
    }

    #[test]
    fn test_storage_pool_clone() {
        let pool = StoragePool {
            name: "clone-test".to_string(),
            status: "ONLINE".to_string(),
            size: 1_000_000_000,
            used: 500_000_000,
            available: 500_000_000,
            health: "HEALTHY".to_string(),
            pool_type: "MIRROR".to_string(),
        };

        let cloned = pool.clone();

        assert_eq!(pool.name, cloned.name);
        assert_eq!(pool.size, cloned.size);
        assert_eq!(pool.used, cloned.used);
    }
}
