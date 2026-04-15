// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! `PoolSetupConfig` construction, properties, tier mappings, and representative topology mixes.

use nestgate_zfs::pool_setup::{
    ConfigDeviceType, ConfigStorageTier, DeviceDetectionConfig, PoolSetupConfig, PoolTopology,
    RedundancyLevel,
};
use std::collections::HashMap;

#[test]
fn test_pool_setup_config_default() {
    let config = PoolSetupConfig::default();

    assert!(!config.pool_name.is_empty());
    assert!(config.devices.is_empty());
}

#[test]
fn test_pool_setup_config_with_devices() {
    let devices = vec![
        "/dev/sda".to_string(),
        "/dev/sdb".to_string(),
        "/dev/sdc".to_string(),
    ];

    let config = PoolSetupConfig {
        pool_name: "test_pool".to_string(),
        devices: devices.clone(),
        topology: PoolTopology::RaidZ1,
        properties: HashMap::new(),
        tier_mappings: HashMap::new(),
        redundancy: RedundancyLevel::Single,
        device_detection: DeviceDetectionConfig::default(),
        create_tiers: false,
    };

    assert_eq!(config.devices.len(), 3);
}

#[test]
fn test_pool_setup_config_with_properties() {
    let mut properties = HashMap::new();
    properties.insert("compression".to_string(), "lz4".to_string());
    properties.insert("atime".to_string(), "off".to_string());

    let config = PoolSetupConfig {
        pool_name: "test_pool".to_string(),
        devices: Vec::new(),
        topology: PoolTopology::Single,
        properties,
        tier_mappings: HashMap::new(),
        redundancy: RedundancyLevel::None,
        device_detection: DeviceDetectionConfig::default(),
        create_tiers: false,
    };

    assert_eq!(config.properties.len(), 2);
    assert_eq!(
        config.properties.get("compression"),
        Some(&"lz4".to_string())
    );
}

#[test]
fn test_pool_setup_config_with_tier_mappings() {
    let mut tier_mappings = HashMap::new();
    tier_mappings.insert(
        ConfigStorageTier::Hot,
        vec![ConfigDeviceType::OptaneMemory, ConfigDeviceType::NvmeSsd],
    );
    tier_mappings.insert(
        ConfigStorageTier::Cold,
        vec![ConfigDeviceType::SpinningDisk],
    );

    let config = PoolSetupConfig {
        pool_name: "test_pool".to_string(),
        devices: Vec::new(),
        topology: PoolTopology::Single,
        properties: HashMap::new(),
        tier_mappings,
        redundancy: RedundancyLevel::None,
        device_detection: DeviceDetectionConfig::default(),
        create_tiers: true,
    };

    assert_eq!(config.tier_mappings.len(), 2);
    assert!(config.create_tiers);
}

#[test]
fn test_pool_setup_config_clone() {
    let config1 = PoolSetupConfig::default();
    let config2 = config1.clone();

    assert_eq!(config1.pool_name, config2.pool_name);
}

#[test]
fn test_pool_setup_config_serialization() {
    let config = PoolSetupConfig::default();
    let json = serde_json::to_string(&config).expect("Should serialize");
    assert!(json.contains("pool_name"));
}

// ==================== CONFIGURATION COMBINATIONS ====================

#[test]
fn test_config_single_optane() {
    let mut tier_mappings = HashMap::new();
    tier_mappings.insert(ConfigStorageTier::Hot, vec![ConfigDeviceType::OptaneMemory]);

    let config = PoolSetupConfig {
        pool_name: "optane_pool".to_string(),
        devices: vec!["/dev/nvme0n1".to_string()],
        topology: PoolTopology::Single,
        properties: HashMap::new(),
        tier_mappings,
        redundancy: RedundancyLevel::None,
        device_detection: DeviceDetectionConfig::default(),
        create_tiers: true,
    };

    assert!(config.create_tiers);
}

#[test]
fn test_config_mirror_nvme() {
    let config = PoolSetupConfig {
        pool_name: "mirror_pool".to_string(),
        devices: vec!["/dev/nvme0n1".to_string(), "/dev/nvme1n1".to_string()],
        topology: PoolTopology::Mirror,
        properties: HashMap::new(),
        tier_mappings: HashMap::new(),
        redundancy: RedundancyLevel::Single,
        device_detection: DeviceDetectionConfig::default(),
        create_tiers: false,
    };

    assert_eq!(config.devices.len(), 2);
}

#[test]
fn test_config_raidz2_mixed() {
    let devices = vec![
        "/dev/sda".to_string(),
        "/dev/sdb".to_string(),
        "/dev/sdc".to_string(),
        "/dev/sdd".to_string(),
        "/dev/sde".to_string(),
        "/dev/sdf".to_string(),
    ];

    let config = PoolSetupConfig {
        pool_name: "raidz2_pool".to_string(),
        devices,
        topology: PoolTopology::RaidZ2,
        properties: HashMap::new(),
        tier_mappings: HashMap::new(),
        redundancy: RedundancyLevel::Double,
        device_detection: DeviceDetectionConfig::default(),
        create_tiers: false,
    };

    assert_eq!(config.devices.len(), 6);
}
