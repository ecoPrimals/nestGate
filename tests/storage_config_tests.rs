// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![allow(
    unused,
    dead_code,
    deprecated,
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::restriction,
    clippy::cargo
)]

//! Comprehensive tests for storage configuration

use anyhow::Result;

#[test]
fn test_storage_config_default() {
    use nestgate_core::config::environment::StorageConfig;

    let config = StorageConfig::default();

    assert!(!config.zfs_pool.is_empty());
    assert!(!config.data_dir.is_empty());
    assert!(config.cache_size_mb > 0);
    assert_eq!(config.snapshot_retention_days, 30);
}

#[test]
fn test_storage_config_compression_enabled_by_default() {
    use nestgate_core::config::environment::StorageConfig;

    let config = StorageConfig::default();
    assert!(
        config.compression_enabled,
        "Compression should be enabled by default"
    );
}

#[test]
fn test_storage_config_cache_size() {
    use nestgate_core::config::environment::StorageConfig;

    let config = StorageConfig::default();
    assert_eq!(
        config.cache_size_mb, 512,
        "Default cache size should be 512MB"
    );
}

#[test]
fn test_storage_config_zfs_pool_default() {
    use nestgate_core::config::environment::StorageConfig;

    let config = StorageConfig::default();
    assert_eq!(config.zfs_pool, "tank", "Default ZFS pool should be 'tank'");
}

#[test]
fn test_storage_config_data_dir_default() {
    use nestgate_core::config::environment::StorageConfig;

    let config = StorageConfig::default();
    // XDG-compliant: defaults to ~/.local/share/nestgate when HOME set,
    // or /var/lib/nestgate when not. Either is valid.
    assert!(
        config.data_dir.ends_with("nestgate"),
        "Default data dir should end with 'nestgate', got: {}",
        config.data_dir
    );
    assert!(!config.data_dir.is_empty());
}

#[test]
fn test_storage_config_is_send() {
    use nestgate_core::config::environment::StorageConfig;

    fn assert_send<T: Send>() {}
    assert_send::<StorageConfig>();
}

#[test]
fn test_storage_config_is_sync() {
    use nestgate_core::config::environment::StorageConfig;

    fn assert_sync<T: Sync>() {}
    assert_sync::<StorageConfig>();
}

#[test]
fn test_storage_config_clone() {
    use nestgate_core::config::environment::StorageConfig;

    let config1 = StorageConfig::default();
    let config2 = config1.clone();

    assert_eq!(config1.zfs_pool, config2.zfs_pool);
    assert_eq!(config1.data_dir, config2.data_dir);
    assert_eq!(config1.cache_size_mb, config2.cache_size_mb);
    assert_eq!(config1.compression_enabled, config2.compression_enabled);
    assert_eq!(
        config1.snapshot_retention_days,
        config2.snapshot_retention_days
    );
}

#[test]
fn test_storage_config_debug() {
    use nestgate_core::config::environment::StorageConfig;

    let config = StorageConfig::default();
    let debug_str = format!("{:?}", config);

    assert!(debug_str.contains("StorageConfig"));
}

#[test]
fn test_storage_config_custom_values() {
    use nestgate_core::config::environment::StorageConfig;

    let config = StorageConfig {
        zfs_pool: "mypool".to_string(),
        cache_size_mb: 1024,
        compression_enabled: false,
        snapshot_retention_days: 60,
        ..Default::default()
    };

    assert_eq!(config.zfs_pool, "mypool");
    assert_eq!(config.cache_size_mb, 1024);
    assert!(!config.compression_enabled);
    assert_eq!(config.snapshot_retention_days, 60);
}

#[test]
fn test_storage_config_large_cache() {
    use nestgate_core::config::environment::StorageConfig;

    let config = StorageConfig {
        cache_size_mb: 4096, // 4GB
        ..Default::default()
    };

    assert_eq!(config.cache_size_mb, 4096);
}

#[test]
fn test_storage_config_small_cache() {
    use nestgate_core::config::environment::StorageConfig;

    let config = StorageConfig {
        cache_size_mb: 128, // 128MB
        ..Default::default()
    };

    assert_eq!(config.cache_size_mb, 128);
}

#[test]
fn test_storage_config_custom_data_dir() {
    use nestgate_core::config::environment::StorageConfig;

    let config = StorageConfig {
        data_dir: "/custom/path/to/data".to_string(),
        ..Default::default()
    };

    assert_eq!(config.data_dir, "/custom/path/to/data");
}

#[test]
fn test_storage_config_retention_bounds() {
    use nestgate_core::config::environment::StorageConfig;

    // Test various retention periods
    let config1 = StorageConfig {
        snapshot_retention_days: 7,
        ..Default::default()
    };
    assert_eq!(config1.snapshot_retention_days, 7);

    let config2 = StorageConfig {
        snapshot_retention_days: 90,
        ..Default::default()
    };
    assert_eq!(config2.snapshot_retention_days, 90);

    let config3 = StorageConfig {
        snapshot_retention_days: 365,
        ..Default::default()
    };
    assert_eq!(config3.snapshot_retention_days, 365);
}

#[test]
fn test_storage_config_serialization() -> Result<()> {
    use nestgate_core::config::environment::StorageConfig;

    let config = StorageConfig::default();

    // Serialize
    let json = serde_json::to_string(&config)?;
    assert!(json.contains("tank"));

    // Deserialize
    let config2: StorageConfig = serde_json::from_str(&json)?;
    assert_eq!(config.zfs_pool, config2.zfs_pool);

    Ok(())
}

#[test]
fn test_storage_config_multiple_instances() {
    use nestgate_core::config::environment::StorageConfig;

    let config1 = StorageConfig::default();
    let config2 = StorageConfig::default();

    // Both should have same defaults
    assert_eq!(config1.zfs_pool, config2.zfs_pool);
    assert_eq!(config1.cache_size_mb, config2.cache_size_mb);
}

#[test]
fn test_storage_config_compression_toggle() {
    use nestgate_core::config::environment::StorageConfig;

    let mut config = StorageConfig::default();

    // Start enabled
    assert!(config.compression_enabled);

    // Disable
    config.compression_enabled = false;
    assert!(!config.compression_enabled);

    // Re-enable
    config.compression_enabled = true;
    assert!(config.compression_enabled);
}

#[test]
fn test_storage_config_zero_cache() {
    use nestgate_core::config::environment::StorageConfig;

    let config = StorageConfig {
        cache_size_mb: 0, // Disable cache
        ..Default::default()
    };

    assert_eq!(config.cache_size_mb, 0);
}

#[test]
fn test_storage_config_long_retention() {
    use nestgate_core::config::environment::StorageConfig;

    let config = StorageConfig {
        snapshot_retention_days: 3650, // 10 years
        ..Default::default()
    };

    assert_eq!(config.snapshot_retention_days, 3650);
}
