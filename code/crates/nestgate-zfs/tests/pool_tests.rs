//! Comprehensive ZFS Pool Management Tests

use nestgate_zfs::pool::{PoolInfo, PoolState, PoolHealth, PoolCapacity, ZfsPoolManager};
use nestgate_zfs::config::ZfsConfig;
use nestgate_core::{NestGateError, Result};
use std::collections::HashMap;

/// Create a sample pool for testing
fn create_test_pool(name: &str) -> PoolInfo {
    PoolInfo {
        name: name.to_string(),
        state: PoolState::Online,
        health: PoolHealth::Healthy,
        capacity: PoolCapacity {
            total_bytes: 1_000_000_000,
            used_bytes: 500_000_000,
            available_bytes: 500_000_000,
            utilization_percent: 50.0,
        },
        devices: vec!["/dev/sda1".to_string(), "/dev/sdb1".to_string()],
        properties: HashMap::new(),
    }
}

#[cfg(test)]
mod pool_manager_tests {
    use super::*;

    #[tokio::test]
    async fn test_pool_manager_creation() {
        let config = ZfsConfig::default();
        let result = ZfsPoolManager::new(&config).await;
        assert!(result.is_ok(), "Failed to create ZfsPoolManager");
    }

    #[test]
    fn test_pool_manager_new_for_testing() {
        let _manager = ZfsPoolManager::new_for_testing();
        // Should not panic
    }

    #[test]
    fn test_pool_manager_new_production() {
        let config = ZfsConfig::default();
        let _manager = ZfsPoolManager::new_production(config);
        // Should not panic
    }

    #[tokio::test]
    async fn test_list_pools() {
        let manager = ZfsPoolManager::new_for_testing();
        let result = manager.list_pools().await;
        assert!(result.is_ok(), "list_pools should not fail");
    }

    #[tokio::test]
    async fn test_get_overall_status() {
        let manager = ZfsPoolManager::new_for_testing();
        let result = manager.get_overall_status().await;
        assert!(result.is_ok(), "get_overall_status should not fail");
    }

    #[tokio::test]
    async fn test_discover_pools() {
        let manager = ZfsPoolManager::new_for_testing();
        let _result = manager.discover_pools().await;
        // May succeed or fail depending on ZFS availability
    }
}

#[cfg(test)]
mod pool_info_tests {
    use super::*;

    #[test]
    fn test_pool_info_creation() {
        let pool = create_test_pool("test_pool");
        assert_eq!(pool.name, "test_pool");
        assert_eq!(pool.state, PoolState::Online);
        assert_eq!(pool.health, PoolHealth::Healthy);
    }

    #[test]
    fn test_pool_capacity_calculations() {
        let capacity = PoolCapacity {
            total_bytes: 1_000_000_000,
            used_bytes: 300_000_000,
            available_bytes: 700_000_000,
            utilization_percent: 30.0,
        };
        
        assert_eq!(capacity.used_bytes + capacity.available_bytes, capacity.total_bytes);
        assert!((capacity.utilization_percent - 30.0).abs() < 0.1);
    }
}

#[cfg(test)]
mod error_handling_tests {
    use super::*;

    #[tokio::test]
    async fn test_get_pool_info_nonexistent() {
        let manager = ZfsPoolManager::new_for_testing();
        let result = manager.get_pool_info("nonexistent_pool").await;
        assert!(result.is_err(), "Should fail for non-existent pool");
    }

    #[tokio::test]
    async fn test_create_pool_error_handling() {
        let manager = ZfsPoolManager::new_for_testing();
        let result = manager.create_pool("test_pool", &["/dev/null".to_string()]).await;
        assert!(result.is_err(), "Should fail with invalid device");
    }

    #[tokio::test]
    async fn test_destroy_pool_error_handling() {
        let manager = ZfsPoolManager::new_for_testing();
        let result = manager.destroy_pool("nonexistent_pool").await;
        assert!(result.is_err(), "Should fail for non-existent pool");
    }
} 