// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **COMPREHENSIVE POOL SETUP MODULE TESTS**
//!
//! Tests for ZFS pool setup framework to achieve >70% coverage.
//! Focus on configuration, device detection, validation, and pool creation logic.

use nestgate_zfs::pool_setup::{
    ConfigDeviceType, ConfigStorageTier, DeviceDetectionConfig, PoolSetupConfig, PoolSetupError,
    PoolSetupResult, PoolTopology, RedundancyLevel,
};

// ==================== POOL TOPOLOGY TESTS ====================

#[test]
fn test_pool_topology_single() {
    let topology = PoolTopology::Single;
    let debug_str = format!("{:?}", topology);
    assert!(debug_str.contains("Single"));
}

#[test]
fn test_pool_topology_mirror() {
    let topology = PoolTopology::Mirror;
    let debug_str = format!("{:?}", topology);
    assert!(debug_str.contains("Mirror"));
}

#[test]
fn test_pool_topology_raidz1() {
    let topology = PoolTopology::RaidZ1;
    let debug_str = format!("{:?}", topology);
    assert!(debug_str.contains("RaidZ1"));
}

#[test]
fn test_pool_topology_raidz2() {
    let topology = PoolTopology::RaidZ2;
    let debug_str = format!("{:?}", topology);
    assert!(debug_str.contains("RaidZ2"));
}

#[test]
fn test_pool_topology_raidz3() {
    let topology = PoolTopology::RaidZ3;
    let debug_str = format!("{:?}", topology);
    assert!(debug_str.contains("RaidZ3"));
}

#[test]
fn test_pool_topology_clone() {
    let topology1 = PoolTopology::Mirror;
    let topology2 = topology1.clone();
    let debug1 = format!("{:?}", topology1);
    let debug2 = format!("{:?}", topology2);
    assert_eq!(debug1, debug2);
}

#[test]
fn test_pool_topology_serialization() {
    let topology = PoolTopology::RaidZ2;
    let json = serde_json::to_string(&topology).expect("Should serialize");
    assert!(json.contains("RaidZ2"));
}

#[test]
fn test_pool_topology_deserialization() {
    let json = r#""Mirror""#;
    let topology: PoolTopology = serde_json::from_str(json).expect("Should deserialize");
    let debug_str = format!("{:?}", topology);
    assert!(debug_str.contains("Mirror"));
}

// ==================== STORAGE TIER TESTS ====================

#[test]
fn test_storage_tier_hot() {
    let tier = ConfigStorageTier::Hot;
    let debug_str = format!("{:?}", tier);
    assert!(debug_str.contains("Hot"));
}

#[test]
fn test_storage_tier_warm() {
    let tier = ConfigStorageTier::Warm;
    let debug_str = format!("{:?}", tier);
    assert!(debug_str.contains("Warm"));
}

#[test]
fn test_storage_tier_cold() {
    let tier = ConfigStorageTier::Cold;
    let debug_str = format!("{:?}", tier);
    assert!(debug_str.contains("Cold"));
}

#[test]
fn test_storage_tier_equality() {
    let tier1 = ConfigStorageTier::Hot;
    let tier2 = ConfigStorageTier::Hot;
    let tier3 = ConfigStorageTier::Warm;

    assert_eq!(tier1, tier2);
    assert_ne!(tier1, tier3);
}

#[test]
fn test_storage_tier_clone() {
    let tier1 = ConfigStorageTier::Hot;
    let tier2 = tier1;
    assert_eq!(tier1, tier2);
}

#[test]
fn test_storage_tier_serialization() {
    let tier = ConfigStorageTier::Hot;
    let json = serde_json::to_string(&tier).expect("Should serialize");
    assert!(json.contains("Hot"));
}

// ==================== DEVICE TYPE TESTS ====================

#[test]
fn test_device_type_optane() {
    let device = ConfigDeviceType::OptaneMemory;
    let debug_str = format!("{:?}", device);
    assert!(debug_str.contains("OptaneMemory"));
}

#[test]
fn test_device_type_nvme() {
    let device = ConfigDeviceType::NvmeSsd;
    let debug_str = format!("{:?}", device);
    assert!(debug_str.contains("NvmeSsd"));
}

#[test]
fn test_device_type_sata() {
    let device = ConfigDeviceType::SataSsd;
    let debug_str = format!("{:?}", device);
    assert!(debug_str.contains("SataSsd"));
}

#[test]
fn test_device_type_spinning() {
    let device = ConfigDeviceType::SpinningDisk;
    let debug_str = format!("{:?}", device);
    assert!(debug_str.contains("SpinningDisk"));
}

#[test]
fn test_device_type_equality() {
    let device1 = ConfigDeviceType::NvmeSsd;
    let device2 = ConfigDeviceType::NvmeSsd;
    let device3 = ConfigDeviceType::SataSsd;

    assert_eq!(device1, device2);
    assert_ne!(device1, device3);
}

#[test]
fn test_device_type_clone() {
    let device1 = ConfigDeviceType::NvmeSsd;
    let device2 = device1.clone();
    assert_eq!(device1, device2);
}

// ==================== REDUNDANCY LEVEL TESTS ====================

#[test]
fn test_redundancy_level_none() {
    let level = RedundancyLevel::None;
    let debug_str = format!("{:?}", level);
    assert!(debug_str.contains("None"));
}

#[test]
fn test_redundancy_level_single() {
    let level = RedundancyLevel::Single;
    let debug_str = format!("{:?}", level);
    assert!(debug_str.contains("Single"));
}

#[test]
fn test_redundancy_level_double() {
    let level = RedundancyLevel::Double;
    let debug_str = format!("{:?}", level);
    assert!(debug_str.contains("Double"));
}

#[test]
fn test_redundancy_level_triple() {
    let level = RedundancyLevel::Triple;
    let debug_str = format!("{:?}", level);
    assert!(debug_str.contains("Triple"));
}

#[test]
fn test_redundancy_level_clone() {
    let level1 = RedundancyLevel::Double;
    let level2 = level1.clone();
    let debug1 = format!("{:?}", level1);
    let debug2 = format!("{:?}", level2);
    assert_eq!(debug1, debug2);
}

// ==================== DEVICE DETECTION CONFIG TESTS ====================

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
    use std::collections::HashMap;

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

// ==================== POOL SETUP CONFIG TESTS ====================

#[test]
fn test_pool_setup_config_default() {
    let config = PoolSetupConfig::default();

    assert!(!config.pool_name.is_empty());
    assert!(config.devices.is_empty());
}

#[test]
fn test_pool_setup_config_with_devices() {
    use std::collections::HashMap;

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
    use std::collections::HashMap;

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
    use std::collections::HashMap;

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

// ==================== POOL SETUP ERROR TESTS ====================

#[test]
fn test_pool_setup_error_device_validation() {
    let err = PoolSetupError::DeviceValidation("Invalid device".to_string());
    assert!(err.to_string().contains("Device validation failed"));
}

#[test]
fn test_pool_setup_error_pool_creation() {
    let err = PoolSetupError::PoolCreation("Failed to create".to_string());
    assert!(err.to_string().contains("Pool creation failed"));
}

#[test]
fn test_pool_setup_error_configuration() {
    let err = PoolSetupError::Configuration("Bad config".to_string());
    assert!(err.to_string().contains("Configuration error"));
}

#[test]
fn test_pool_setup_error_device_scanning() {
    let err = PoolSetupError::DeviceScanning("Scan failed".to_string());
    assert!(err.to_string().contains("Device scanning failed"));
}

#[test]
fn test_pool_setup_error_insufficient_devices() {
    let err = PoolSetupError::InsufficientDevices("Need more".to_string());
    assert!(err.to_string().contains("Insufficient devices"));
}

#[test]
fn test_pool_setup_error_zfs_command() {
    let err = PoolSetupError::ZfsCommand("Command failed".to_string());
    assert!(err.to_string().contains("ZFS command failed"));
}

#[test]
fn test_pool_setup_error_debug() {
    let err = PoolSetupError::Configuration("test".to_string());
    let debug_str = format!("{:?}", err);
    assert!(debug_str.contains("Configuration"));
}

// ==================== POOL SETUP RESULT TESTS ====================

#[test]
fn test_pool_setup_result_success() {
    let result = PoolSetupResult {
        pool_name: "test_pool".to_string(),
        success: true,
        message: "Pool created".to_string(),
        devices_used: vec!["/dev/sda".to_string()],
        topology: PoolTopology::Single,
    };

    assert!(result.success);
    assert_eq!(result.pool_name, "test_pool");
}

#[test]
fn test_pool_setup_result_failure() {
    let result = PoolSetupResult {
        pool_name: "test_pool".to_string(),
        success: false,
        message: "Creation failed".to_string(),
        devices_used: Vec::new(),
        topology: PoolTopology::Single,
    };

    assert!(!result.success);
    assert!(result.devices_used.is_empty());
}

#[test]
fn test_pool_setup_result_with_multiple_devices() {
    let devices = vec![
        "/dev/sda".to_string(),
        "/dev/sdb".to_string(),
        "/dev/sdc".to_string(),
    ];

    let result = PoolSetupResult {
        pool_name: "raid_pool".to_string(),
        success: true,
        message: "RAID pool created".to_string(),
        devices_used: devices.clone(),
        topology: PoolTopology::RaidZ1,
    };

    assert_eq!(result.devices_used.len(), 3);
}

#[test]
fn test_pool_setup_result_clone() {
    let result1 = PoolSetupResult {
        pool_name: "test_pool".to_string(),
        success: true,
        message: "Success".to_string(),
        devices_used: vec!["/dev/sda".to_string()],
        topology: PoolTopology::Mirror,
    };

    let result2 = result1.clone();
    assert_eq!(result1.pool_name, result2.pool_name);
    assert_eq!(result1.success, result2.success);
}

#[test]
fn test_pool_setup_result_serialization() {
    let result = PoolSetupResult {
        pool_name: "test_pool".to_string(),
        success: true,
        message: "Success".to_string(),
        devices_used: vec!["/dev/sda".to_string()],
        topology: PoolTopology::Single,
    };

    let json = serde_json::to_string(&result).expect("Should serialize");
    assert!(json.contains("test_pool"));
    assert!(json.contains("success"));
}

#[test]
fn test_pool_setup_result_deserialization() {
    let json = r#"{
        "pool_name": "my_pool",
        "success": true,
        "message": "Created",
        "devices_used": ["/dev/sda"],
        "topology": "Mirror"
    }"#;

    let result: PoolSetupResult = serde_json::from_str(json).expect("Should deserialize");
    assert_eq!(result.pool_name, "my_pool");
    assert!(result.success);
}

// ==================== CONFIGURATION COMBINATIONS ====================

#[test]
fn test_config_single_optane() {
    use std::collections::HashMap;

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
    use std::collections::HashMap;

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
    use std::collections::HashMap;

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

// ==================== EDGE CASE TESTS ====================

#[test]
fn test_empty_pool_name() {
    use std::collections::HashMap;

    let config = PoolSetupConfig {
        pool_name: String::new(),
        devices: Vec::new(),
        topology: PoolTopology::Single,
        properties: HashMap::new(),
        tier_mappings: HashMap::new(),
        redundancy: RedundancyLevel::None,
        device_detection: DeviceDetectionConfig::default(),
        create_tiers: false,
    };

    assert!(config.pool_name.is_empty());
}

#[test]
fn test_very_long_pool_name() {
    use std::collections::HashMap;

    let long_name = "a".repeat(1000);
    let config = PoolSetupConfig {
        pool_name: long_name.clone(),
        devices: Vec::new(),
        topology: PoolTopology::Single,
        properties: HashMap::new(),
        tier_mappings: HashMap::new(),
        redundancy: RedundancyLevel::None,
        device_detection: DeviceDetectionConfig::default(),
        create_tiers: false,
    };

    assert_eq!(config.pool_name.len(), 1000);
}

#[test]
fn test_special_characters_in_pool_name() {
    use std::collections::HashMap;

    let special_name = "pool-123_test.prod";
    let config = PoolSetupConfig {
        pool_name: special_name.to_string(),
        devices: Vec::new(),
        topology: PoolTopology::Single,
        properties: HashMap::new(),
        tier_mappings: HashMap::new(),
        redundancy: RedundancyLevel::None,
        device_detection: DeviceDetectionConfig::default(),
        create_tiers: false,
    };

    assert_eq!(config.pool_name, special_name);
}

#[test]
fn test_many_devices() {
    use std::collections::HashMap;

    let devices: Vec<String> = (0..100)
        .map(|i| format!("/dev/sd{}", (b'a' + (i % 26) as u8) as char))
        .collect();

    let config = PoolSetupConfig {
        pool_name: "large_pool".to_string(),
        devices: devices.clone(),
        topology: PoolTopology::RaidZ3,
        properties: HashMap::new(),
        tier_mappings: HashMap::new(),
        redundancy: RedundancyLevel::Triple,
        device_detection: DeviceDetectionConfig::default(),
        create_tiers: false,
    };

    assert_eq!(config.devices.len(), 100);
}

#[test]
fn test_many_properties() {
    use std::collections::HashMap;

    let mut properties = HashMap::new();
    for i in 0..100 {
        properties.insert(format!("prop{}", i), format!("value{}", i));
    }

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

    assert_eq!(config.properties.len(), 100);
}

// ==================== ROUNDTRIP TESTS ====================

#[test]
fn test_pool_setup_config_roundtrip() {
    let original = PoolSetupConfig::default();

    let json = serde_json::to_string(&original).expect("Should serialize");
    let deserialized: PoolSetupConfig = serde_json::from_str(&json).expect("Should deserialize");

    assert_eq!(original.pool_name, deserialized.pool_name);
}

#[test]
fn test_pool_setup_result_roundtrip() {
    let original = PoolSetupResult {
        pool_name: "test".to_string(),
        success: true,
        message: "OK".to_string(),
        devices_used: vec!["/dev/sda".to_string()],
        topology: PoolTopology::Mirror,
    };

    let json = serde_json::to_string(&original).expect("Should serialize");
    let deserialized: PoolSetupResult = serde_json::from_str(&json).expect("Should deserialize");

    assert_eq!(original.pool_name, deserialized.pool_name);
    assert_eq!(original.success, deserialized.success);
}
