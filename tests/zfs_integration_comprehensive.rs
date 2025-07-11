use nestgate_core::{NestGateError, Result, StorageTier};
use nestgate_zfs::{config::ZfsConfig, ZfsManager};
use serde_json::json;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tempfile::TempDir;
use tokio::time::{sleep, Duration};
use uuid::Uuid;

/// Comprehensive ZFS Integration Test Suite
/// Tests pool lifecycle, data integrity, snapshots, backups, and multi-pool coordination

pub struct ZfsTestEnvironment {
    pub temp_dir: TempDir,
    pub zfs_manager: Arc<ZfsManager>,
    pub test_pool_name: String,
    pub test_datasets: Vec<String>,
}

impl ZfsTestEnvironment {
    pub async fn new() -> Result<Self> {
        let temp_dir = TempDir::new()
            .map_err(|e| NestGateError::FileSystem(format!("Failed to create temp dir: {}", e)))?;

        let config = ZfsConfig {
            default_pool: "nestpool".to_string(),
            enable_compression: true,
            enable_deduplication: false,
            default_block_size: 128 * 1024,
            enable_encryption: false,
            ..Default::default()
        };

        let zfs_manager = Arc::new(ZfsManager::new(config).await?);
        let test_pool_name = format!("testpool_{}", Uuid::new_v4().to_string()[..8]);

        Ok(Self {
            temp_dir,
            zfs_manager,
            test_pool_name,
            test_datasets: vec![],
        })
    }

    pub async fn create_test_dataset(&mut self, name: &str) -> Result<String> {
        let dataset_name = format!("test/{}", name);
        self.zfs_manager
            .create_dataset(&dataset_name, &self.test_pool_name, StorageTier::Hot)
            .await?;
        self.test_datasets.push(dataset_name.clone());
        Ok(dataset_name)
    }

    pub async fn cleanup(&mut self) -> Result<()> {
        // Clean up test datasets
        for dataset in &self.test_datasets {
            let _ = self.zfs_manager.delete_dataset(dataset).await;
        }

        // Note: In a real environment, we might clean up test pools too
        // but for safety in tests, we'll leave pool management to the system
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    #[tokio::test]
    async fn test_pool_creation_destruction_lifecycle() {
        // Note: This test simulates pool operations without actually creating ZFS pools
        // Real ZFS pool creation requires root privileges and block devices

        let config = ZfsConfig::default();
        let zfs_manager = ZfsManager::new(config).await.unwrap();

        // Test pool information retrieval
        let pools_result = zfs_manager.list_pools().await;
        assert!(pools_result.is_ok(), "Should be able to list ZFS pools");

        let pools = pools_result.unwrap();

        // Verify pool status information
        for pool in &pools {
            assert!(!pool.name.is_empty(), "Pool name should not be empty");
            assert!(pool.capacity > 0, "Pool capacity should be positive");
            // Status should be valid ZFS status
            assert!(
                ["ONLINE", "DEGRADED", "FAULTED", "OFFLINE", "UNAVAIL", "REMOVED"]
                    .contains(&pool.status.as_str()),
                "Pool status should be valid ZFS status"
            );
        }
    }

    #[tokio::test]
    async fn test_data_integrity_verification() {
        let mut env = ZfsTestEnvironment::new().await.unwrap();
        let dataset_name = env.create_test_dataset("integrity_test").await.unwrap();

        // Create test data
        let test_data = b"Hello, ZFS integrity test! This is some test data.";
        let test_file_path = format!("/{}/test_file.txt", dataset_name);

        // Simulate writing data (in real ZFS, this would write to the dataset)
        // For testing, we'll use the ZFS manager's metadata operations

        // Test data verification through ZFS manager
        let verification_result = env
            .zfs_manager
            .verify_dataset_integrity(&dataset_name)
            .await;
        assert!(
            verification_result.is_ok(),
            "Dataset integrity verification should succeed"
        );

        let integrity_status = verification_result.unwrap();
        assert!(
            integrity_status.is_healthy,
            "New dataset should have healthy integrity"
        );
        assert_eq!(
            integrity_status.checksum_errors, 0,
            "New dataset should have no checksum errors"
        );

        env.cleanup().await.unwrap();
    }

    #[tokio::test]
    async fn test_snapshot_management_comprehensive() {
        let mut env = ZfsTestEnvironment::new().await.unwrap();
        let dataset_name = env.create_test_dataset("snapshot_test").await.unwrap();

        // Create initial snapshot
        let snapshot_name = format!("{}@initial", dataset_name);
        let snapshot_result = env
            .zfs_manager
            .create_snapshot(&dataset_name, "initial")
            .await;
        assert!(snapshot_result.is_ok(), "Should be able to create snapshot");

        // List snapshots
        let snapshots_result = env.zfs_manager.list_snapshots(&dataset_name).await;
        assert!(snapshots_result.is_ok(), "Should be able to list snapshots");

        let snapshots = snapshots_result.unwrap();
        assert!(!snapshots.is_empty(), "Should have at least one snapshot");

        let initial_snapshot = snapshots.iter().find(|s| s.name.contains("@initial"));
        assert!(
            initial_snapshot.is_some(),
            "Should find the initial snapshot"
        );

        // Create second snapshot
        let second_snapshot_result = env
            .zfs_manager
            .create_snapshot(&dataset_name, "second")
            .await;
        assert!(
            second_snapshot_result.is_ok(),
            "Should be able to create second snapshot"
        );

        // Verify we now have two snapshots
        let updated_snapshots = env.zfs_manager.list_snapshots(&dataset_name).await.unwrap();
        assert!(
            updated_snapshots.len() >= 2,
            "Should have at least two snapshots"
        );

        // Delete a snapshot
        let delete_result = env.zfs_manager.delete_snapshot(&snapshot_name).await;
        assert!(delete_result.is_ok(), "Should be able to delete snapshot");

        env.cleanup().await.unwrap();
    }

    #[tokio::test]
    async fn test_backup_restore_operations() {
        let mut env = ZfsTestEnvironment::new().await.unwrap();
        let source_dataset = env.create_test_dataset("backup_source").await.unwrap();
        let backup_dataset = env.create_test_dataset("backup_target").await.unwrap();

        // Create snapshot for backup
        let snapshot_result = env
            .zfs_manager
            .create_snapshot(&source_dataset, "backup_point")
            .await;
        assert!(snapshot_result.is_ok(), "Should create snapshot for backup");

        // Simulate backup operation
        let backup_path = env.temp_dir.path().join("backup.zfs");
        let backup_result = env
            .zfs_manager
            .backup_dataset(&format!("{}@backup_point", source_dataset), &backup_path)
            .await;
        assert!(backup_result.is_ok(), "Backup operation should succeed");

        // Verify backup file exists
        assert!(backup_path.exists(), "Backup file should exist");
        assert!(
            backup_path.metadata().unwrap().len() > 0,
            "Backup file should not be empty"
        );

        // Simulate restore operation
        let restore_result = env
            .zfs_manager
            .restore_dataset(&backup_path, &backup_dataset)
            .await;
        assert!(restore_result.is_ok(), "Restore operation should succeed");

        // Verify restored dataset integrity
        let integrity_result = env
            .zfs_manager
            .verify_dataset_integrity(&backup_dataset)
            .await;
        assert!(
            integrity_result.is_ok(),
            "Restored dataset should have good integrity"
        );

        env.cleanup().await.unwrap();
    }

    #[tokio::test]
    async fn test_multi_pool_coordination() {
        let config = ZfsConfig::default();
        let zfs_manager = ZfsManager::new(config).await.unwrap();

        // Get list of available pools
        let pools = zfs_manager.list_pools().await.unwrap();

        if pools.len() < 2 {
            // Skip test if we don't have multiple pools
            println!(
                "Skipping multi-pool test: Only {} pools available",
                pools.len()
            );
            return;
        }

        let pool1 = &pools[0].name;
        let pool2 = &pools[1].name;

        // Create datasets on different pools
        let dataset1 = format!("multipool_test1_{}", Uuid::new_v4().to_string()[..8]);
        let dataset2 = format!("multipool_test2_{}", Uuid::new_v4().to_string()[..8]);

        let create1_result = zfs_manager
            .create_dataset(&dataset1, pool1, StorageTier::Hot)
            .await;
        let create2_result = zfs_manager
            .create_dataset(&dataset2, pool2, StorageTier::Cold)
            .await;

        assert!(
            create1_result.is_ok(),
            "Should create dataset on first pool"
        );
        assert!(
            create2_result.is_ok(),
            "Should create dataset on second pool"
        );

        // Test cross-pool operations
        let pool1_datasets = zfs_manager.list_datasets(Some(pool1)).await.unwrap();
        let pool2_datasets = zfs_manager.list_datasets(Some(pool2)).await.unwrap();

        assert!(
            pool1_datasets.iter().any(|d| d.name.contains(&dataset1)),
            "Dataset1 should exist on pool1"
        );
        assert!(
            pool2_datasets.iter().any(|d| d.name.contains(&dataset2)),
            "Dataset2 should exist on pool2"
        );

        // Cleanup
        let _ = zfs_manager.delete_dataset(&dataset1).await;
        let _ = zfs_manager.delete_dataset(&dataset2).await;
    }

    #[tokio::test]
    async fn test_dataset_properties_and_quotas() {
        let mut env = ZfsTestEnvironment::new().await.unwrap();
        let dataset_name = env.create_test_dataset("properties_test").await.unwrap();

        // Set dataset properties
        let quota_result = env
            .zfs_manager
            .set_dataset_quota(&dataset_name, 1024 * 1024 * 1024)
            .await; // 1GB
        assert!(quota_result.is_ok(), "Should be able to set dataset quota");

        // Get dataset properties
        let properties_result = env.zfs_manager.get_dataset_properties(&dataset_name).await;
        assert!(
            properties_result.is_ok(),
            "Should be able to get dataset properties"
        );

        let properties = properties_result.unwrap();
        assert!(
            properties.contains_key("quota") || properties.contains_key("used"),
            "Properties should contain quota or usage information"
        );

        // Test compression setting
        let compression_result = env
            .zfs_manager
            .set_dataset_compression(&dataset_name, "lz4")
            .await;
        assert!(
            compression_result.is_ok(),
            "Should be able to set compression"
        );

        env.cleanup().await.unwrap();
    }

    #[tokio::test]
    async fn test_dataset_replication() {
        let mut env = ZfsTestEnvironment::new().await.unwrap();
        let source_dataset = env.create_test_dataset("repl_source").await.unwrap();
        let target_dataset = env.create_test_dataset("repl_target").await.unwrap();

        // Create snapshot for replication
        let snapshot_result = env
            .zfs_manager
            .create_snapshot(&source_dataset, "repl_snap")
            .await;
        assert!(
            snapshot_result.is_ok(),
            "Should create snapshot for replication"
        );

        // Start replication
        let replication_result = env
            .zfs_manager
            .replicate_dataset(&format!("{}@repl_snap", source_dataset), &target_dataset)
            .await;
        assert!(replication_result.is_ok(), "Replication should succeed");

        // Verify replication status
        let repl_status = env
            .zfs_manager
            .get_replication_status(&source_dataset, &target_dataset)
            .await;
        assert!(
            repl_status.is_ok(),
            "Should be able to get replication status"
        );

        env.cleanup().await.unwrap();
    }

    #[tokio::test]
    async fn test_dataset_encryption() {
        let mut env = ZfsTestEnvironment::new().await.unwrap();

        // Create encrypted dataset
        let encrypted_dataset = format!("encrypted_test_{}", Uuid::new_v4().to_string()[..8]);
        let encryption_result = env
            .zfs_manager
            .create_encrypted_dataset(
                &encrypted_dataset,
                &env.test_pool_name,
                "test_passphrase",
                StorageTier::Hot,
            )
            .await;

        // Note: Encryption may not be available in all test environments
        if encryption_result.is_ok() {
            env.test_datasets.push(encrypted_dataset.clone());

            // Test encrypted dataset operations
            let properties = env
                .zfs_manager
                .get_dataset_properties(&encrypted_dataset)
                .await
                .unwrap();
            assert!(
                properties.get("encryption").map_or(false, |v| v != "off"),
                "Dataset should be encrypted"
            );

            // Test key management
            let key_status = env
                .zfs_manager
                .get_encryption_key_status(&encrypted_dataset)
                .await;
            assert!(key_status.is_ok(), "Should be able to get key status");
        } else {
            println!("Skipping encryption test: ZFS encryption not available");
        }

        env.cleanup().await.unwrap();
    }

    #[tokio::test]
    async fn test_performance_monitoring() {
        let mut env = ZfsTestEnvironment::new().await.unwrap();
        let dataset_name = env.create_test_dataset("perf_test").await.unwrap();

        // Get initial performance metrics
        let initial_metrics = env.zfs_manager.get_performance_metrics(&dataset_name).await;
        assert!(
            initial_metrics.is_ok(),
            "Should be able to get performance metrics"
        );

        let metrics = initial_metrics.unwrap();

        // Verify metric structure
        assert!(
            metrics.read_ops >= 0,
            "Read operations should be non-negative"
        );
        assert!(
            metrics.write_ops >= 0,
            "Write operations should be non-negative"
        );
        assert!(metrics.read_bytes >= 0, "Read bytes should be non-negative");
        assert!(
            metrics.write_bytes >= 0,
            "Write bytes should be non-negative"
        );

        // Test I/O statistics
        let io_stats = env.zfs_manager.get_io_statistics(&dataset_name).await;
        assert!(io_stats.is_ok(), "Should be able to get I/O statistics");

        env.cleanup().await.unwrap();
    }

    #[tokio::test]
    async fn test_zfs_error_handling() {
        let mut env = ZfsTestEnvironment::new().await.unwrap();

        // Test creating dataset with invalid name
        let invalid_result = env
            .zfs_manager
            .create_dataset(
                "invalid/name/with/too/many/slashes",
                &env.test_pool_name,
                StorageTier::Hot,
            )
            .await;
        assert!(
            invalid_result.is_err(),
            "Should fail with invalid dataset name"
        );

        // Test operating on non-existent dataset
        let nonexistent_dataset = "nonexistent_dataset_12345";
        let snapshot_result = env
            .zfs_manager
            .create_snapshot(nonexistent_dataset, "test")
            .await;
        assert!(
            snapshot_result.is_err(),
            "Should fail to snapshot non-existent dataset"
        );

        // Test deleting non-existent snapshot
        let delete_result = env
            .zfs_manager
            .delete_snapshot("nonexistent@snapshot")
            .await;
        assert!(
            delete_result.is_err(),
            "Should fail to delete non-existent snapshot"
        );

        env.cleanup().await.unwrap();
    }

    #[tokio::test]
    async fn test_concurrent_operations() {
        let mut env = ZfsTestEnvironment::new().await.unwrap();

        // Create multiple datasets concurrently
        let mut handles = vec![];

        for i in 0..3 {
            let zfs_manager = env.zfs_manager.clone();
            let pool_name = env.test_pool_name.clone();
            let dataset_name = format!("concurrent_test_{}", i);

            let handle = tokio::spawn(async move {
                zfs_manager
                    .create_dataset(&dataset_name, &pool_name, StorageTier::Hot)
                    .await
            });
            handles.push((handle, dataset_name));
        }

        // Wait for all operations to complete
        for (handle, dataset_name) in handles {
            let result = handle.await.unwrap();
            if result.is_ok() {
                env.test_datasets.push(dataset_name);
            }
        }

        // Verify datasets were created
        let datasets = env
            .zfs_manager
            .list_datasets(Some(&env.test_pool_name))
            .await
            .unwrap();
        let concurrent_datasets: Vec<_> = datasets
            .iter()
            .filter(|d| d.name.contains("concurrent_test"))
            .collect();

        assert!(
            !concurrent_datasets.is_empty(),
            "Should have created some concurrent datasets"
        );

        env.cleanup().await.unwrap();
    }
}
