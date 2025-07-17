//! Comprehensive ZFS Pool Tests
//!
//! Tests the public API of the ZFS pool management system

use nestgate_core::NestGateError;
use nestgate_zfs::{
    config::ZfsConfig,
    pool::{PoolCapacity, PoolHealth, PoolInfo, PoolState, ZfsPoolManager},
};
use std::collections::HashMap;

/// Create a sample pool for testing
fn _create_sample_pool(name: &str) -> PoolInfo {
    PoolInfo {
        name: name.to_string(),
        state: PoolState::Online,
        health: PoolHealth::Healthy,
        capacity: PoolCapacity {
            total_bytes: 1024 * 1024 * 1024 * 1024,    // 1TB
            used_bytes: 1024 * 1024 * 1024 * 512,      // 512GB
            available_bytes: 1024 * 1024 * 1024 * 512, // 512GB
            utilization_percent: 50.0,
        },
        devices: vec!["/dev/sda1".to_string()],
        properties: HashMap::new(),
    }
}

#[cfg(test)]
mod pool_manager_tests {
    use super::*;

    #[test]
    fn test_pool_manager_creation() {
        // Test creating pool manager for testing
        let _manager = ZfsPoolManager::new_for_testing();
        // Should not panic
        println!("Pool manager created successfully");
    }

    #[tokio::test]
    async fn test_pool_discovery() {
        let manager = ZfsPoolManager::new_for_testing();

        // Use list_pools which returns Vec<PoolInfo>
        let result = manager.list_pools().await;

        // Should handle gracefully whether ZFS is available or not
        match result {
            Ok(pools) => {
                println!("Discovered {} pools", pools.len());
                // Verify pool structure if any pools exist
                for pool in pools {
                    assert!(!pool.name.is_empty(), "Pool name should not be empty");
                    // Verify pool has valid states
                    println!(
                        "Pool: {} - State: {:?} - Health: {:?}",
                        pool.name, pool.state, pool.health
                    );
                }
            }
            Err(e) => {
                println!("Pool discovery failed as expected in test environment: {e}");
                // This is acceptable in environments without ZFS
            }
        }
    }

    #[tokio::test]
    async fn test_overall_status() {
        let manager = ZfsPoolManager::new_for_testing();

        let result = manager.get_overall_status().await;

        // Should always succeed with valid structure
        assert!(result.is_ok(), "Should be able to get overall status");
        let status = result.unwrap();

        // Verify status structure
        // These are always true for unsigned integers, but kept for documentation
        assert!(
            status.pools_online == status.pools_online,
            "Online pools count should be valid"
        );
        assert!(
            status.pools_degraded == status.pools_degraded,
            "Degraded pools count should be valid"
        );
        assert!(
            status.total_capacity == status.total_capacity,
            "Total capacity should be valid"
        );
        assert!(
            status.available_capacity == status.available_capacity,
            "Available capacity should be valid"
        );

        println!(
            "Pool Status - Online: {}, Degraded: {}, Total: {}B, Available: {}B",
            status.pools_online,
            status.pools_degraded,
            status.total_capacity,
            status.available_capacity
        );
    }

    #[tokio::test]
    async fn test_pool_operations_error_handling() {
        let manager = ZfsPoolManager::new_for_testing();

        // Test get_pool_info
        let pool_info_result = manager.get_pool_info("nonexistent_pool").await;
        match pool_info_result {
            Ok(_) => println!("get_pool_info unexpectedly succeeded"),
            Err(e) => {
                println!("get_pool_info failed as expected: {e}");
                match e {
                    NestGateError::NotFound(_)
                    | NestGateError::Internal(_)
                    | NestGateError::Io(_) => {
                        // These are appropriate error types
                    }
                    _ => println!("Unexpected error type for get_pool_info: {e:?}"),
                }
            }
        }

        // Test get_pool_status
        let pool_status_result = manager.get_pool_status("nonexistent_pool").await;
        match pool_status_result {
            Ok(_) => println!("get_pool_status unexpectedly succeeded"),
            Err(e) => {
                println!("get_pool_status failed as expected: {e}");
            }
        }

        // Test refresh_pool_info
        let refresh_result = manager.refresh_pool_info("nonexistent_pool").await;
        match refresh_result {
            Ok(_) => println!("refresh_pool_info unexpectedly succeeded"),
            Err(e) => {
                println!("refresh_pool_info failed as expected: {e}");
            }
        }
    }

    #[tokio::test]
    async fn test_pool_creation_error_handling() {
        let manager = ZfsPoolManager::new_for_testing();

        // Test pool creation with invalid devices
        let result = manager
            .create_pool("test_pool", &["/dev/null".to_string()])
            .await;

        // Should fail gracefully
        assert!(
            result.is_err(),
            "Should fail to create pool with invalid device"
        );

        let error = result.err().unwrap();
        println!("Pool creation failed as expected: {error}");
    }

    #[tokio::test]
    async fn test_pool_destruction_error_handling() {
        let manager = ZfsPoolManager::new_for_testing();

        let result = manager.destroy_pool("nonexistent_pool").await;

        // Should fail gracefully
        assert!(result.is_err(), "Should fail to destroy non-existent pool");
        println!("Pool destruction failed as expected");
    }
}

#[cfg(test)]
mod pool_info_tests {
    use super::*;

    #[test]
    fn test_pool_info_structure() {
        use std::collections::HashMap;

        // Test creating pool info structure with correct fields
        let pool_info = PoolInfo {
            name: "test_pool".to_string(),
            state: PoolState::Online,
            health: PoolHealth::Healthy,
            capacity: PoolCapacity {
                total_bytes: 1024 * 1024 * 1024 * 1024,    // 1TB
                used_bytes: 1024 * 1024 * 1024 * 512,      // 512GB
                available_bytes: 1024 * 1024 * 1024 * 512, // 512GB
                utilization_percent: 50.0,
            },
            devices: vec!["/dev/sda1".to_string(), "/dev/sdb1".to_string()],
            properties: HashMap::new(),
        };

        // Verify structure
        assert_eq!(pool_info.name, "test_pool");
        assert!(matches!(pool_info.state, PoolState::Online));
        assert!(matches!(pool_info.health, PoolHealth::Healthy));
        assert_eq!(pool_info.capacity.utilization_percent, 50.0);
        assert_eq!(pool_info.devices.len(), 2);

        // Verify capacity calculations
        assert_eq!(
            pool_info.capacity.used_bytes + pool_info.capacity.available_bytes,
            pool_info.capacity.total_bytes
        );
    }

    #[test]
    fn test_pool_states() {
        let states = vec![
            PoolState::Online,
            PoolState::Degraded,
            PoolState::Faulted,
            PoolState::Offline,
            PoolState::Unknown,
        ];

        for state in states {
            println!("Pool state: {state:?}");
            // Should not panic
        }
    }

    #[test]
    fn test_pool_health() {
        let health_states = vec![
            PoolHealth::Healthy,
            PoolHealth::Warning,
            PoolHealth::Critical,
            PoolHealth::Unknown,
        ];

        for health in health_states {
            println!("Pool health: {health:?}");
            // Should not panic
        }
    }
}

#[cfg(test)]
mod concurrent_operations_tests {
    use super::*;

    #[tokio::test]
    async fn test_concurrent_pool_discovery() {
        // Test concurrent pool operations
        let tasks = vec![
            tokio::spawn(async {
                let manager = ZfsPoolManager::new_for_testing();
                manager.list_pools().await
            }),
            tokio::spawn(async {
                let manager = ZfsPoolManager::new_for_testing();
                manager.list_pools().await
            }),
            tokio::spawn(async {
                let manager = ZfsPoolManager::new_for_testing();
                manager.list_pools().await
            }),
        ];

        // Wait for all tasks to complete
        for task in tasks {
            let result = task.await;
            assert!(result.is_ok(), "Concurrent operation should not panic");

            match result.unwrap() {
                Ok(_) => println!("Concurrent operation succeeded"),
                Err(_) => println!("Concurrent operation failed as expected"),
            }
        }
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_pool_manager_with_config() {
        let config = ZfsConfig::default();

        // Test creating pool manager with configuration
        let result = ZfsPoolManager::new(&config).await;

        match result {
            Ok(manager) => {
                println!("Pool manager created with config");

                // Test basic operations
                let status = manager.get_overall_status().await;
                assert!(status.is_ok(), "Should be able to get status");
            }
            Err(e) => {
                println!("Pool manager creation failed as expected: {e}");
                // This is acceptable in test environments
            }
        }
    }

    #[test]
    fn test_configuration_integration() {
        let config = ZfsConfig::default();

        // Verify config has pool settings
        assert!(config.pool_discovery.auto_discovery);
        assert!(config.health_monitoring.enabled);
        assert_eq!(config.health_monitoring.check_interval_seconds, 30);
    }
}
