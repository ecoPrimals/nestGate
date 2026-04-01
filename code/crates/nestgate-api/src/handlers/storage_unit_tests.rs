// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Unit tests for storage handlers
//!
//! These tests cover the storage handler data structures and basic operations
//! without requiring actual ZFS installation.

#[cfg(test)]
mod tests {
    use super::super::storage::*;

    // ==================== StorageHandler Tests ====================

    #[test]
    fn test_storage_handler_new() {
        let _handler = StorageHandler::new();
        // Handler creation should succeed (smoke test)
    }

    #[test]
    fn test_storage_handler_default() {
        let _handler = StorageHandler;
        // Default should create valid handler (smoke test)
    }

    #[test]
    fn test_storage_handler_clone() {
        let handler1 = StorageHandler::new();
        let _handler2 = handler1;
        // Both handlers should be valid (smoke test)
    }

    // ==================== StoragePool Tests ====================

    #[test]
    fn test_storage_pool_creation() {
        let pool = StoragePool {
            name: "testpool".to_string(),
            status: "ONLINE".to_string(),
            size: 1_000_000_000,
            used: 500_000_000,
            available: 500_000_000,
            health: "HEALTHY".to_string(),
            pool_type: "raidz".to_string(),
        };

        assert_eq!(pool.name, "testpool");
        assert_eq!(pool.status, "ONLINE");
        assert_eq!(pool.size, 1_000_000_000);
        assert_eq!(pool.used, 500_000_000);
        assert_eq!(pool.available, 500_000_000);
    }

    #[test]
    fn test_storage_pool_clone() {
        let pool1 = StoragePool {
            name: "testpool".to_string(),
            status: "ONLINE".to_string(),
            size: 1_000_000_000,
            used: 500_000_000,
            available: 500_000_000,
            health: "HEALTHY".to_string(),
            pool_type: "raidz".to_string(),
        };

        let pool2 = pool1.clone();
        assert_eq!(pool1.name, pool2.name);
        assert_eq!(pool1.size, pool2.size);
    }

    #[test]
    fn test_storage_pool_serialization() {
        let pool = StoragePool {
            name: "testpool".to_string(),
            status: "ONLINE".to_string(),
            size: 1_000_000_000,
            used: 500_000_000,
            available: 500_000_000,
            health: "HEALTHY".to_string(),
            pool_type: "raidz".to_string(),
        };

        // Test serialization
        let json = serde_json::to_string(&pool).expect("Failed to serialize");
        assert!(json.contains("testpool"));
        assert!(json.contains("ONLINE"));
    }

    #[test]
    fn test_storage_pool_deserialization() {
        let json = r#"{
            "name": "testpool",
            "status": "ONLINE",
            "size": 1000000000,
            "used": 500000000,
            "available": 500000000,
            "health": "HEALTHY",
            "pool_type": "raidz"
        }"#;

        let pool: StoragePool = serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(pool.name, "testpool");
        assert_eq!(pool.size, 1_000_000_000);
    }

    #[test]
    fn test_storage_pool_capacity_calculation() {
        let pool = StoragePool {
            name: "testpool".to_string(),
            status: "ONLINE".to_string(),
            size: 1_000_000_000,
            used: 500_000_000,
            available: 500_000_000,
            health: "HEALTHY".to_string(),
            pool_type: "raidz".to_string(),
        };

        // Verify capacity calculations
        assert_eq!(pool.used + pool.available, pool.size);
    }

    // ==================== StorageDataset Tests ====================

    #[test]
    fn test_storage_dataset_creation() {
        let dataset = StorageDataset {
            name: "testpool/dataset1".to_string(),
            pool: "testpool".to_string(),
            size: 500_000_000,
            used: 250_000_000,
            available: 250_000_000,
            mount_point: "/mnt/testpool/dataset1".to_string(),
            compression: "lz4".to_string(),
        };

        assert_eq!(dataset.name, "testpool/dataset1");
        assert_eq!(dataset.pool, "testpool");
        assert_eq!(dataset.compression, "lz4");
    }

    #[test]
    fn test_storage_dataset_clone() {
        let dataset1 = StorageDataset {
            name: "testpool/dataset1".to_string(),
            pool: "testpool".to_string(),
            size: 500_000_000,
            used: 250_000_000,
            available: 250_000_000,
            mount_point: "/mnt/testpool/dataset1".to_string(),
            compression: "lz4".to_string(),
        };

        let dataset2 = dataset1.clone();
        assert_eq!(dataset1.name, dataset2.name);
        assert_eq!(dataset1.mount_point, dataset2.mount_point);
    }

    #[test]
    fn test_storage_dataset_serialization() {
        let dataset = StorageDataset {
            name: "testpool/dataset1".to_string(),
            pool: "testpool".to_string(),
            size: 500_000_000,
            used: 250_000_000,
            available: 250_000_000,
            mount_point: "/mnt/testpool/dataset1".to_string(),
            compression: "lz4".to_string(),
        };

        let json = serde_json::to_string(&dataset).expect("Failed to serialize");
        assert!(json.contains("testpool/dataset1"));
        assert!(json.contains("lz4"));
    }

    #[test]
    fn test_storage_dataset_deserialization() {
        let json = r#"{
            "name": "testpool/dataset1",
            "pool": "testpool",
            "size": 500000000,
            "used": 250000000,
            "available": 250000000,
            "mount_point": "/mnt/testpool/dataset1",
            "compression": "lz4"
        }"#;

        let dataset: StorageDataset = serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(dataset.name, "testpool/dataset1");
        assert_eq!(dataset.compression, "lz4");
    }

    // ==================== StorageSnapshot Tests ====================

    #[test]
    fn test_storage_snapshot_creation() {
        let snapshot = StorageSnapshot {
            name: "testpool/dataset1@snap1".to_string(),
            dataset: "testpool/dataset1".to_string(),
            size: 100_000_000,
            created: "2024-11-07T12:00:00Z".to_string(),
            referenced: 100_000_000,
        };

        assert_eq!(snapshot.name, "testpool/dataset1@snap1");
        assert_eq!(snapshot.dataset, "testpool/dataset1");
        assert_eq!(snapshot.size, 100_000_000);
    }

    #[test]
    fn test_storage_snapshot_clone() {
        let snap1 = StorageSnapshot {
            name: "testpool/dataset1@snap1".to_string(),
            dataset: "testpool/dataset1".to_string(),
            size: 100_000_000,
            created: "2024-11-07T12:00:00Z".to_string(),
            referenced: 100_000_000,
        };

        let snap2 = snap1.clone();
        assert_eq!(snap1.name, snap2.name);
        assert_eq!(snap1.created, snap2.created);
    }

    #[test]
    fn test_storage_snapshot_serialization() {
        let snapshot = StorageSnapshot {
            name: "testpool/dataset1@snap1".to_string(),
            dataset: "testpool/dataset1".to_string(),
            size: 100_000_000,
            created: "2024-11-07T12:00:00Z".to_string(),
            referenced: 100_000_000,
        };

        let json = serde_json::to_string(&snapshot).expect("Failed to serialize");
        assert!(json.contains("testpool/dataset1@snap1"));
        assert!(json.contains("2024-11-07"));
    }

    #[test]
    fn test_storage_snapshot_deserialization() {
        let json = r#"{
            "name": "testpool/dataset1@snap1",
            "dataset": "testpool/dataset1",
            "size": 100000000,
            "created": "2024-11-07T12:00:00Z",
            "referenced": 100000000
        }"#;

        let snapshot: StorageSnapshot = serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(snapshot.name, "testpool/dataset1@snap1");
        assert_eq!(snapshot.size, 100_000_000);
    }

    // ==================== StorageMetrics Tests ====================

    #[test]
    fn test_storage_metrics_creation() {
        let metrics = StorageMetrics {
            total_pools: 5,
            total_datasets: 20,
            total_snapshots: 100,
            total_storage: 10_000_000_000,
            used_storage: 6_000_000_000,
            available_storage: 4_000_000_000,
            iops: 1500.0,
            bandwidth_mbps: 250.5,
            health_status: "HEALTHY".to_string(),
        };

        assert_eq!(metrics.total_pools, 5);
        assert_eq!(metrics.total_datasets, 20);
        assert_eq!(metrics.total_snapshots, 100);
        assert_eq!(metrics.health_status, "HEALTHY");
    }

    #[test]
    fn test_storage_metrics_clone() {
        let metrics1 = StorageMetrics {
            total_pools: 5,
            total_datasets: 20,
            total_snapshots: 100,
            total_storage: 10_000_000_000,
            used_storage: 6_000_000_000,
            available_storage: 4_000_000_000,
            iops: 1500.0,
            bandwidth_mbps: 250.5,
            health_status: "HEALTHY".to_string(),
        };

        let metrics2 = metrics1.clone();
        assert_eq!(metrics1.total_pools, metrics2.total_pools);
        assert_eq!(metrics1.iops, metrics2.iops);
    }

    #[test]
    fn test_storage_metrics_serialization() {
        let metrics = StorageMetrics {
            total_pools: 5,
            total_datasets: 20,
            total_snapshots: 100,
            total_storage: 10_000_000_000,
            used_storage: 6_000_000_000,
            available_storage: 4_000_000_000,
            iops: 1500.0,
            bandwidth_mbps: 250.5,
            health_status: "HEALTHY".to_string(),
        };

        let json = serde_json::to_string(&metrics).expect("Failed to serialize");
        assert!(json.contains("\"total_pools\":5"));
        assert!(json.contains("HEALTHY"));
    }

    #[test]
    fn test_storage_metrics_deserialization() {
        let json = r#"{
            "total_pools": 5,
            "total_datasets": 20,
            "total_snapshots": 100,
            "total_storage": 10000000000,
            "used_storage": 6000000000,
            "available_storage": 4000000000,
            "iops": 1500.0,
            "bandwidth_mbps": 250.5,
            "health_status": "HEALTHY"
        }"#;

        let metrics: StorageMetrics = serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(metrics.total_pools, 5);
        assert_eq!(metrics.iops, 1500.0);
    }

    #[test]
    fn test_storage_metrics_capacity_validation() {
        let metrics = StorageMetrics {
            total_pools: 5,
            total_datasets: 20,
            total_snapshots: 100,
            total_storage: 10_000_000_000,
            used_storage: 6_000_000_000,
            available_storage: 4_000_000_000,
            iops: 1500.0,
            bandwidth_mbps: 250.5,
            health_status: "HEALTHY".to_string(),
        };

        // Verify storage math
        assert_eq!(
            metrics.used_storage + metrics.available_storage,
            metrics.total_storage
        );
    }

    #[test]
    fn test_storage_metrics_utilization_percentage() {
        let metrics = StorageMetrics {
            total_pools: 5,
            total_datasets: 20,
            total_snapshots: 100,
            total_storage: 10_000_000_000,
            used_storage: 6_000_000_000,
            available_storage: 4_000_000_000,
            iops: 1500.0,
            bandwidth_mbps: 250.5,
            health_status: "HEALTHY".to_string(),
        };

        let utilization = (metrics.used_storage as f64 / metrics.total_storage as f64) * 100.0;
        assert!((utilization - 60.0).abs() < 0.01);
    }

    // ==================== Edge Cases ====================

    #[test]
    fn test_storage_pool_zero_size() {
        let pool = StoragePool {
            name: "empty".to_string(),
            status: "OFFLINE".to_string(),
            size: 0,
            used: 0,
            available: 0,
            health: "DEGRADED".to_string(),
            pool_type: "none".to_string(),
        };

        assert_eq!(pool.size, 0);
        assert_eq!(pool.used + pool.available, 0);
    }

    #[test]
    fn test_storage_dataset_empty_mount_point() {
        let dataset = StorageDataset {
            name: "testpool/dataset1".to_string(),
            pool: "testpool".to_string(),
            size: 500_000_000,
            used: 250_000_000,
            available: 250_000_000,
            mount_point: String::new(),
            compression: "off".to_string(),
        };

        assert_eq!(dataset.mount_point, "");
    }

    #[test]
    fn test_storage_metrics_zero_iops() {
        let metrics = StorageMetrics {
            total_pools: 0,
            total_datasets: 0,
            total_snapshots: 0,
            total_storage: 0,
            used_storage: 0,
            available_storage: 0,
            iops: 0.0,
            bandwidth_mbps: 0.0,
            health_status: "UNKNOWN".to_string(),
        };

        assert_eq!(metrics.iops, 0.0);
        assert_eq!(metrics.bandwidth_mbps, 0.0);
    }

    // ==================== Debug Trait Tests ====================

    #[test]
    fn test_storage_pool_debug_format() {
        let pool = StoragePool {
            name: "testpool".to_string(),
            status: "ONLINE".to_string(),
            size: 1_000_000_000,
            used: 500_000_000,
            available: 500_000_000,
            health: "HEALTHY".to_string(),
            pool_type: "raidz".to_string(),
        };

        let debug_str = format!("{pool:?}");
        assert!(debug_str.contains("testpool"));
        assert!(debug_str.contains("ONLINE"));
    }

    #[test]
    fn test_storage_dataset_debug_format() {
        let dataset = StorageDataset {
            name: "testpool/dataset1".to_string(),
            pool: "testpool".to_string(),
            size: 500_000_000,
            used: 250_000_000,
            available: 250_000_000,
            mount_point: "/mnt/testpool/dataset1".to_string(),
            compression: "lz4".to_string(),
        };

        let debug_str = format!("{dataset:?}");
        assert!(debug_str.contains("testpool/dataset1"));
        assert!(debug_str.contains("lz4"));
    }
}
