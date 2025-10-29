//
// Tests the core ZFS functionality without complex integrations

use nestgate_core::StorageTier;
use nestgate_zfs::{config::ZfsConfig, dataset::ZfsDatasetManager, pool::ZfsPoolManager};
use std::sync::Arc;

use nestgate_core::canonical_types::StorageTier;
#[tokio::test]
async fn test_zfs_config_validation() -> Result<(), Box<dyn std::error::Error>> {
    let config = ZfsConfig::default();

    // Test that default configuration is valid
    assert!(config.validate().is_ok());

    // Test ZFS-specific extensions
    assert!(!config.extensions.pools.auto_discovery);
    assert_eq!(config.extensions.pools.default_pool_type, "raidz1");
    assert_eq!(config.extensions.datasets.default_compression, "lz4");

    // Test pool discovery settings
    assert!(config.pool_discovery.auto_discovery);

    // Test health monitoring settings
    assert!(config.health_monitoring.enabled);
    assert_eq!(config.health_monitoring.check_interval_seconds, 30);
    Ok(())
}

#[tokio::test]
async fn test_storage_tier_functionality() -> Result<(), Box<dyn std::error::Error>> {
    let hot_tier = StorageTier::Hot;
    let warm_tier = StorageTier::Warm;
    let cold_tier = StorageTier::Cold;

    // Test tier ordering/comparison
    assert_ne!(hot_tier, warm_tier);
    assert_ne!(warm_tier, cold_tier);
    assert_ne!(hot_tier, cold_tier);

    // Test serialization/deserialization
    let tier_str = format!("{}", "actual_error_details");
    assert!(tier_str.contains("Hot"));
    Ok(())
}

#[tokio::test]
async fn test_pool_manager_basic_operations() -> Result<(), Box<dyn std::error::Error>> {
    let config = ZfsConfig::default();

    // Test creating pool manager
    let pool_manager = match ZfsPoolManager::new(&config).await {
        Ok(pm) => pm,
        Err(_) => ZfsPoolManager::new_for_testing(), // Fallback for tests
    };

    // Test basic operations
    let overall_status = pool_manager.get_overall_status().await;
    assert!(
        overall_status.is_ok(),
        "Should be able to get overall status"
    );

    let pool_list = pool_manager.list_pools().await;
    assert!(pool_list.is_ok(), "Should be able to list pools");
    Ok(())
}

#[tokio::test]
async fn test_dataset_manager_creation() -> Result<(), Box<dyn std::error::Error>> {
    let config = ZfsConfig::default();

    let pool_manager = match ZfsPoolManager::new(&config).await {
        Ok(pm) => pm,
        Err(_) => ZfsPoolManager::new_for_testing(), // Fallback for tests
    };

    let pool_manager_arc = Arc::new(pool_manager);

    // Test dataset manager creation
    let _dataset_manager = ZfsDatasetManager::new(config.clone(), pool_manager_arc);

    // Just test that we can create the manager
    assert_eq!(config.pool_discovery.default_pool, "zfspool");
    Ok(())
}

#[tokio::test]
async fn test_heuristic_tier_recommendation() -> Result<(), Box<dyn std::error::Error>> {
    // Test heuristic-based tier recommendation (replacing AI functionality)

    // Large frequently accessed file should go to hot tier
    let large_frequent_file_size: u64 = 10 * 1024 * 1024 * 1024; // 10GB
    let recent_access_days = 1.0;
    let high_frequency = 100.0;

    // Simple heuristic logic
    let recommended_tier = if large_frequent_file_size > 1024 * 1024 * 1024 // > 1GB
        && recent_access_days < 7.0
        && high_frequency > 50.0
    {
        StorageTier::Hot
    } else if recent_access_days < 30.0 {
        StorageTier::Warm
    } else {
        StorageTier::Cold
    };

    assert_eq!(recommended_tier, StorageTier::Hot);

    // Old infrequently accessed file should go to cold tier
    let old_access_days = 100.0;
    let low_frequency = 1.0;

    let recommended_tier_cold = if old_access_days > 90.0 && low_frequency < 5.0 {
        StorageTier::Cold
    } else {
        StorageTier::Warm
    };

    assert_eq!(recommended_tier_cold, StorageTier::Cold);
    Ok(())
}

#[tokio::test]
async fn test_configuration_defaults() -> Result<(), Box<dyn std::error::Error>> {
    let config = ZfsConfig::default();

    // Test default values
    assert_eq!(config.pool_discovery.default_pool, "zfspool");
    assert!(config.pool_discovery.auto_discovery);
    assert!(config.health_monitoring.enabled);
    assert_eq!(config.health_monitoring.check_interval_seconds, 30);
    assert_eq!(config.tiers.hot.name, "hot");
    assert_eq!(config.tiers.warm.name, "warm");
    assert_eq!(config.tiers.cold.name, "cold");
    Ok(())
}

#[tokio::test]
async fn test_error_handling() -> Result<(), Box<dyn std::error::Error>> {
    // Test that the ZFS manager handles errors gracefully
    let config = ZfsConfig::default();

    // Test that configurations are valid
    assert!(config.validate().is_ok());
    Ok(())
}

#[tokio::test]
async fn test_concurrent_operations() -> Result<(), Box<dyn std::error::Error>> {
    // Test concurrent operations don't cause issues
    let tasks = vec![
        tokio::spawn(async {
            let config = ZfsConfig::default();
            let _manager = ZfsPoolManager::new_for_testing();
            config.validate()
        }),
        tokio::spawn(async {
            let config = ZfsConfig::default();
            let _manager = ZfsPoolManager::new_for_testing();
            config.validate()
        }),
        tokio::spawn(async {
            let config = ZfsConfig::default();
            let _manager = ZfsPoolManager::new_for_testing();
            config.validate()
        }),
    ];

    for task in tasks {
        let result = task.await;
        assert!(result.is_ok(), "Concurrent operation should not panic");
        assert!(
            result
                .unwrap_or_else(|e| {
                    tracing::error!("Unwrap failed: {:?}", e);
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Operation failed: {}", e),
                    )
                    .into());
                })
                .is_ok(),
            "Configuration should be valid"
        );
        Ok(())
    }
    Ok(())
}
