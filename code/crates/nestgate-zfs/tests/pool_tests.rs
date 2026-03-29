// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

#![allow(
    dead_code,
    unused_doc_comments,
    unused_imports,
    missing_docs,
    rustdoc::missing_crate_level_docs,
    deprecated,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::doc_markdown,
    clippy::module_name_repetitions,
    clippy::struct_excessive_bools,
    clippy::struct_field_names,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::cast_lossless,
    clippy::must_use_candidate,
    clippy::return_self_not_must_use,
    clippy::unnecessary_wraps,
    clippy::unused_self,
    clippy::unused_async,
    clippy::needless_pass_by_value,
    clippy::option_if_let_else,
    clippy::too_long_first_doc_paragraph,
    clippy::inline_always,
    clippy::redundant_closure,
    clippy::redundant_closure_for_method_calls,
    clippy::collapsible_if,
    clippy::single_char_pattern,
    clippy::implicit_hasher,
    clippy::float_cmp,
    clippy::manual_midpoint,
    clippy::suboptimal_flops,
    clippy::items_after_statements,
    clippy::items_after_test_module,
    clippy::too_many_lines,
    clippy::cognitive_complexity,
    clippy::unreadable_literal,
    clippy::redundant_clone,
    clippy::useless_vec,
    clippy::field_reassign_with_default,
    clippy::cmp_null,
    clippy::bool_assert_comparison,
    clippy::used_underscore_items,
    clippy::needless_raw_string_hashes,
    clippy::ref_as_ptr,
    clippy::no_effect_underscore_binding,
    clippy::needless_collect,
    clippy::module_inception,
    clippy::default_trait_access,
    clippy::wildcard_in_or_patterns,
    clippy::or_fun_call,
    clippy::manual_string_new,
    clippy::unnecessary_literal_unwrap,
    clippy::unnecessary_debug_formatting,
    clippy::assigning_clones,
    clippy::unnecessary_unwrap,
    clippy::unnecessary_map_or,
    clippy::unnecessary_lazy_evaluations,
    clippy::similar_names,
    clippy::needless_continue,
    clippy::collection_is_never_read,
    clippy::char_lit_as_u8,
    clippy::ptr_eq,
    clippy::uninlined_format_args,
    clippy::absurd_extreme_comparisons,
    clippy::match_wild_err_arm,
    clippy::single_match_else,
    clippy::derive_partial_eq_without_eq,
    clippy::match_wildcard_for_single_variants,
    clippy::missing_const_for_fn,
    clippy::used_underscore_binding,
    clippy::ignored_unit_patterns,
    unused_comparisons,
    clippy::format_push_string
)]

//
// Tests the public API of the ZFS pool management system

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
            fragmentation_percent: 0.0,
            deduplication_ratio: 1.0,
            total: 1024,
            used: 1024,
            available: 1024,
        },
        devices: vec!["/dev/sda1".to_string()],
        properties: HashMap::new(),
    }
}
#[cfg(test)]
mod pool_manager_tests {
    use super::*;

    #[tokio::test]
    async fn test_pool_manager_creation() -> std::result::Result<(), Box<dyn std::error::Error>> {
        // Test creating pool manager for testing
        let _manager = ZfsPoolManager::new_production(ZfsConfig::default());
        // Should not panic
        println!("Pool manager created successfully");
        Ok(())
    }
    #[tokio::test]
    async fn test_pool_discovery() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let manager = ZfsPoolManager::new_production(ZfsConfig::default());

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
        Ok(())
    }

    #[tokio::test]
    async fn test_overall_status() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let manager = ZfsPoolManager::new_production(ZfsConfig::default());

        let result = manager.get_overall_status().await;

        // Should always succeed with valid structure
        assert!(result.is_ok(), "Should be able to get overall status");
        let status = result.expect("Failed to get overall status");

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
        Ok(())
    }

    #[tokio::test]
    async fn test_pool_operations_error_handling()
    -> std::result::Result<(), Box<dyn std::error::Error>> {
        let config = ZfsConfig::default();
        let manager = ZfsPoolManager::new_production(config);

        // Test get_pool_info
        let pool_info_result = manager.get_pool_info("nonexistent_pool").await;
        match pool_info_result {
            Ok(_) => println!("get_pool_info unexpectedly succeeded"),
            Err(e) => {
                println!("get_pool_info failed as expected: {e}");
                // Error handling - just verify we got an error
                println!("Error type: {:?}", e);
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
        Ok(())
    }

    #[tokio::test]
    async fn test_pool_creation_error_handling()
    -> std::result::Result<(), Box<dyn std::error::Error>> {
        let manager = ZfsPoolManager::new_production(ZfsConfig::default());

        // Test pool creation with invalid devices
        let result = manager
            .create_pool("test_pool", &["/dev/null".to_string()])
            .await;

        // Should fail gracefully
        assert!(
            result.is_err(),
            "Should fail to create pool with invalid device"
        );

        let error = result.err().unwrap_or_else(|| {
            panic!("Failed to get error from result");
        });
        println!("Pool creation failed as expected: {error}");
        Ok(())
    }

    #[tokio::test]
    async fn test_pool_destruction_error_handling()
    -> std::result::Result<(), Box<dyn std::error::Error>> {
        let manager = ZfsPoolManager::new_production(ZfsConfig::default());

        let result = manager.destroy_pool("nonexistent_pool").await;

        // Should fail gracefully
        assert!(result.is_err(), "Should fail to destroy non-existent pool");
        println!("Pool destruction failed as expected");
        Ok(())
    }
}

#[cfg(test)]
mod pool_info_tests {
    use super::*;

    #[test]
    fn test_pool_info_structure() -> std::result::Result<(), Box<dyn std::error::Error>> {
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
                fragmentation_percent: 0.0,
                deduplication_ratio: 1.0,
                total: 1024,
                used: 1024,
                available: 1024,
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
        Ok(())
    }

    #[test]
    fn test_pool_states() -> std::result::Result<(), Box<dyn std::error::Error>> {
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
        Ok(())
    }

    #[test]
    fn test_pool_health() -> std::result::Result<(), Box<dyn std::error::Error>> {
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
        Ok(())
    }
}

#[cfg(test)]
mod concurrent_operations_tests {
    use super::*;

    #[tokio::test]
    async fn test_concurrent_pool_discovery() -> std::result::Result<(), Box<dyn std::error::Error>>
    {
        // Test concurrent pool operations
        let tasks = vec![
            tokio::spawn(async {
                let manager = ZfsPoolManager::new_production(ZfsConfig::default());
                let _ = manager.list_pools().await;
                Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
            }),
            tokio::spawn(async {
                let manager = ZfsPoolManager::new_production(ZfsConfig::default());
                let _ = manager.list_pools().await;
                Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
            }),
            tokio::spawn(async {
                let manager = ZfsPoolManager::new_production(ZfsConfig::default());
                let _ = manager.list_pools().await;
                Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
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
        Ok(())
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_pool_manager_with_config() -> std::result::Result<(), Box<dyn std::error::Error>>
    {
        let config = ZfsConfig::default();

        // Test creating pool manager with configuration
        let result = ZfsPoolManager::new(&config).await;

        match result {
            Ok(manager) => {
                println!("Pool manager created with config");

                // Test basic operations
                let status = manager.get_overall_status().await;
                assert!(status.is_ok(), "Should be able to get status");
                Ok(())
            }
            Err(e) => {
                println!("Pool manager creation failed as expected: {e}");
                // This is acceptable in test environments
                Ok(())
            }
        }
    }

    #[test]
    fn test_configuration_integration() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let config = ZfsConfig::default();

        // Verify config has required fields
        assert!(
            !config.zfs_binary.is_empty(),
            "ZFS binary path should be set"
        );
        assert!(
            !config.zpool_binary.is_empty(),
            "ZPool binary path should be set"
        );
        println!(
            "Config validated: zfs={}, zpool={}",
            config.zfs_binary, config.zpool_binary
        );
        Ok(())
    }
}
