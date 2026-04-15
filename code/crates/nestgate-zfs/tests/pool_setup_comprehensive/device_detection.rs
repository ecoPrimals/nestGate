// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Device scan configuration: defaults, cloning, and JSON round-trips.

use nestgate_zfs::pool_setup::{
    DeviceDetectionConfig, PoolSetupConfig, PoolTopology, RedundancyLevel,
};
use std::collections::HashMap;

#[test]
fn test_device_detection_config_default() {
    let config = DeviceDetectionConfig::default();

    assert!(!config.scan_paths.is_empty());
    assert!(!config.exclude_patterns.is_empty());
    assert!(!config.include_removable);
    assert!(config.min_device_size > 0);
}

#[test]
fn test_device_detection_config_custom() {
    let config = PoolSetupConfig {
        pool_name: "custom_pool".to_string(),
        devices: vec!["/dev/sda".to_string()],
        topology: PoolTopology::Mirror,
        properties: HashMap::new(),
        tier_mappings: HashMap::new(),
        redundancy: RedundancyLevel::Single,
        device_detection: DeviceDetectionConfig::default(),
        create_tiers: false,
    };

    assert_eq!(config.pool_name, "custom_pool");
    assert_eq!(config.devices.len(), 1);
}

#[test]
fn test_device_detection_config_clone() {
    let config1 = DeviceDetectionConfig::default();
    let config2 = config1.clone();

    assert_eq!(config1.scan_paths, config2.scan_paths);
    assert_eq!(config1.min_device_size, config2.min_device_size);
}

#[test]
fn test_device_detection_config_serialization() {
    let config = DeviceDetectionConfig::default();
    let json = serde_json::to_string(&config).expect("Should serialize");
    assert!(json.contains("scan_paths"));
}

#[test]
fn test_device_detection_config_with_custom_paths() {
    let mut config = DeviceDetectionConfig::default();
    config.scan_paths = vec!["/dev/disk/by-id".to_string()];

    assert_eq!(config.scan_paths.len(), 1);
    assert_eq!(config.scan_paths[0], "/dev/disk/by-id");
}

#[test]
fn test_device_detection_config_exclude_patterns() {
    let mut config = DeviceDetectionConfig::default();
    config.exclude_patterns.push("nvme0n1p".to_string());

    assert!(config.exclude_patterns.contains(&"nvme0n1p".to_string()));
}

#[test]
fn test_device_detection_config_size_limits() {
    let mut config = DeviceDetectionConfig::default();
    config.min_device_size = 10 * 1024 * 1024 * 1024; // 10GB
    config.max_device_size = 100 * 1024 * 1024 * 1024; // 100GB

    assert_eq!(config.min_device_size, 10 * 1024 * 1024 * 1024);
    assert_eq!(config.max_device_size, 100 * 1024 * 1024 * 1024);
}
