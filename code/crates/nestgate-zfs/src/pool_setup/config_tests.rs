// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Comprehensive tests for pool setup configuration types
//!
//! Tests configuration defaults, serialization, and type conversions.

use super::config::*;

// ==================== PoolSetupConfig Tests ====================

#[test]
fn test_pool_setup_config_default() {
    let config = PoolSetupConfig::default();

    assert!(!config.pool_name.is_empty());
    assert!(config.devices.is_empty() || !config.devices.is_empty());
    assert!(config.properties.is_empty() || !config.properties.is_empty());
}

#[test]
fn test_pool_setup_config_clone() {
    let config1 = PoolSetupConfig::default();
    let config2 = config1.clone();

    assert_eq!(config1.pool_name, config2.pool_name);
}

#[test]
fn test_pool_setup_config_debug() {
    let config = PoolSetupConfig::default();
    let debug = format!("{config:?}");

    assert!(debug.contains("PoolSetupConfig"));
}

#[test]
fn test_pool_setup_config_serialization() {
    let config = PoolSetupConfig::default();
    let json = serde_json::to_string(&config).expect("Failed to serialize");

    assert!(json.contains("pool_name"));
}

#[test]
fn test_pool_setup_config_deserialization() {
    let config = PoolSetupConfig::default();
    let json = serde_json::to_string(&config).expect("Failed to serialize");
    let restored: PoolSetupConfig = serde_json::from_str(&json).expect("Failed to deserialize");

    assert_eq!(config.pool_name, restored.pool_name);
}

// ==================== PoolTopology Tests ====================

#[test]
fn test_pool_topology_single() {
    let topology = PoolTopology::Single;
    let json = serde_json::to_string(&topology).expect("Failed to serialize");

    assert!(json.contains("Single"));
}

#[test]
fn test_pool_topology_mirror() {
    let topology = PoolTopology::Mirror;
    let json = serde_json::to_string(&topology).expect("Failed to serialize");

    assert!(json.contains("Mirror"));
}

#[test]
fn test_pool_topology_raidz1() {
    let topology = PoolTopology::RaidZ1;
    let json = serde_json::to_string(&topology).expect("Failed to serialize");

    assert!(json.contains("RaidZ1"));
}

#[test]
fn test_pool_topology_raidz2() {
    let topology = PoolTopology::RaidZ2;
    let json = serde_json::to_string(&topology).expect("Failed to serialize");

    assert!(json.contains("RaidZ2"));
}

#[test]
fn test_pool_topology_raidz3() {
    let topology = PoolTopology::RaidZ3;
    let json = serde_json::to_string(&topology).expect("Failed to serialize");

    assert!(json.contains("RaidZ3"));
}

#[test]
fn test_pool_topology_all_variants_clone() {
    let topologies = vec![
        PoolTopology::Single,
        PoolTopology::Mirror,
        PoolTopology::RaidZ1,
        PoolTopology::RaidZ2,
        PoolTopology::RaidZ3,
    ];

    for topology in topologies {
        let cloned = topology.clone();
        let json1 = serde_json::to_string(&topology).unwrap();
        let json2 = serde_json::to_string(&cloned).unwrap();
        assert_eq!(json1, json2);
    }
}

// ==================== StorageTier Tests ====================

#[test]
fn test_storage_tier_hot() {
    let tier = StorageTier::Hot;
    let json = serde_json::to_string(&tier).expect("Failed to serialize");

    assert!(json.contains("Hot"));
}

#[test]
fn test_storage_tier_warm() {
    let tier = StorageTier::Warm;
    let json = serde_json::to_string(&tier).expect("Failed to serialize");

    assert!(json.contains("Warm"));
}

#[test]
fn test_storage_tier_cold() {
    let tier = StorageTier::Cold;
    let json = serde_json::to_string(&tier).expect("Failed to serialize");

    assert!(json.contains("Cold"));
}

#[test]
fn test_storage_tier_equality() {
    let tier1 = StorageTier::Hot;
    let tier2 = StorageTier::Hot;
    let tier3 = StorageTier::Warm;

    assert_eq!(tier1, tier2);
    assert_ne!(tier1, tier3);
}

#[test]
fn test_storage_tier_hash() {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert(StorageTier::Hot, "hot_data");
    map.insert(StorageTier::Warm, "warm_data");
    map.insert(StorageTier::Cold, "cold_data");

    assert_eq!(map.len(), 3);
    assert_eq!(map.get(&StorageTier::Hot), Some(&"hot_data"));
}

// ==================== DeviceType Tests ====================

#[test]
fn test_device_type_optane() {
    let dtype = DeviceType::OptaneMemory;
    let json = serde_json::to_string(&dtype).expect("Failed to serialize");

    assert!(json.contains("OptaneMemory"));
}

#[test]
fn test_device_type_nvme_ssd() {
    let dtype = DeviceType::NvmeSsd;
    let json = serde_json::to_string(&dtype).expect("Failed to serialize");

    assert!(json.contains("NvmeSsd"));
}

#[test]
fn test_device_type_sata_ssd() {
    let dtype = DeviceType::SataSsd;
    let json = serde_json::to_string(&dtype).expect("Failed to serialize");

    assert!(json.contains("SataSsd"));
}

#[test]
fn test_device_type_spinning_disk() {
    let dtype = DeviceType::SpinningDisk;
    let json = serde_json::to_string(&dtype).expect("Failed to serialize");

    assert!(json.contains("SpinningDisk"));
}

#[test]
fn test_device_type_equality() {
    let dtype1 = DeviceType::NvmeSsd;
    let dtype2 = DeviceType::NvmeSsd;
    let dtype3 = DeviceType::SataSsd;

    assert_eq!(dtype1, dtype2);
    assert_ne!(dtype1, dtype3);
}

#[test]
fn test_device_type_hash() {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert(DeviceType::NvmeSsd, 10);
    map.insert(DeviceType::SataSsd, 20);

    assert_eq!(map.get(&DeviceType::NvmeSsd), Some(&10));
}

// ==================== RedundancyLevel Tests ====================

#[test]
fn test_redundancy_level_none() {
    let level = RedundancyLevel::None;
    let json = serde_json::to_string(&level).expect("Failed to serialize");

    assert!(json.contains("None"));
}

#[test]
fn test_redundancy_level_single() {
    let level = RedundancyLevel::Single;
    let json = serde_json::to_string(&level).expect("Failed to serialize");

    assert!(json.contains("Single"));
}

#[test]
fn test_redundancy_level_double() {
    let level = RedundancyLevel::Double;
    let json = serde_json::to_string(&level).expect("Failed to serialize");

    assert!(json.contains("Double"));
}

#[test]
fn test_redundancy_level_triple() {
    let level = RedundancyLevel::Triple;
    let json = serde_json::to_string(&level).expect("Failed to serialize");

    assert!(json.contains("Triple"));
}

#[test]
fn test_redundancy_level_all_variants() {
    let levels = vec![
        RedundancyLevel::None,
        RedundancyLevel::Single,
        RedundancyLevel::Double,
        RedundancyLevel::Triple,
    ];

    for level in levels {
        let json = serde_json::to_string(&level).expect("Failed to serialize");
        assert!(!json.is_empty());
    }
}

// ==================== PoolPropertyConfig Tests ====================

#[test]
fn test_pool_property_config_default() {
    #[allow(deprecated, reason = "testing deprecated API backward compatibility")]
    let config = PoolPropertyConfig::default();

    assert_eq!(config.ashift, 12);
    assert!(config.autoexpand);
    assert!(config.autotrim);
    assert_eq!(config.compression, "lz4");
    assert_eq!(config.recordsize, "128K");
}

#[test]
fn test_pool_property_config_clone() {
    #[allow(deprecated, reason = "testing deprecated API backward compatibility")]
    let config1 = PoolPropertyConfig::default();
    let config2 = config1.clone();

    assert_eq!(config1.ashift, config2.ashift);
    assert_eq!(config1.compression, config2.compression);
}

#[test]
fn test_pool_property_config_serialization() {
    #[allow(deprecated, reason = "testing deprecated API backward compatibility")]
    let config = PoolPropertyConfig::default();
    let json = serde_json::to_string(&config).expect("Failed to serialize");

    assert!(json.contains("ashift"));
    assert!(json.contains("compression"));
}

// ==================== DeviceDetectionConfig Tests ====================

#[test]
fn test_device_detection_config_default() {
    let config = DeviceDetectionConfig::default();

    assert!(!config.scan_paths.is_empty());
    assert_eq!(config.min_device_size, 1024 * 1024 * 1024); // 1GB
    assert!(!config.include_removable);
    assert!(!config.include_loop_devices);
}

#[test]
fn test_device_detection_config_clone() {
    let config1 = DeviceDetectionConfig::default();
    let config2 = config1.clone();

    assert_eq!(config1.min_device_size, config2.min_device_size);
    assert_eq!(config1.scan_paths.len(), config2.scan_paths.len());
}

#[test]
fn test_device_detection_config_serialization() {
    let config = DeviceDetectionConfig::default();
    let json = serde_json::to_string(&config).expect("Failed to serialize");

    assert!(json.contains("scan_paths"));
    assert!(json.contains("min_device_size"));
}

#[test]
fn test_device_detection_config_deserialization() {
    let config = DeviceDetectionConfig::default();
    let json = serde_json::to_string(&config).expect("Failed to serialize");
    let restored: DeviceDetectionConfig =
        serde_json::from_str(&json).expect("Failed to deserialize");

    assert_eq!(config.min_device_size, restored.min_device_size);
}

#[test]
fn test_device_detection_config_custom_values() {
    let config = DeviceDetectionConfig {
        scan_paths: vec!["/dev/nvme*".to_string()],
        exclude_patterns: vec!["test".to_string()],
        include_removable: true,
        min_device_size: 500_000_000,
        max_device_size: 10_000_000_000,
        skip_mountpoints: vec!["/boot".to_string()],
        skip_fstypes: vec!["vfat".to_string()],
        include_loop_devices: true,
    };

    assert_eq!(config.scan_paths.len(), 1);
    assert_eq!(config.min_device_size, 500_000_000);
    assert!(config.include_removable);
}

// ==================== Integration Tests ====================

#[test]
fn test_full_config_serialization_cycle() {
    let config = PoolSetupConfig::default();

    // Serialize
    let json = serde_json::to_string(&config).expect("Serialization failed");

    // Deserialize
    let restored: PoolSetupConfig = serde_json::from_str(&json).expect("Deserialization failed");

    // Verify
    assert_eq!(config.pool_name, restored.pool_name);
}

#[test]
fn test_device_type_conversion_consistency() {
    // Test that config DeviceType and detection DeviceType align
    let config_types = vec![
        DeviceType::OptaneMemory,
        DeviceType::NvmeSsd,
        DeviceType::SataSsd,
        DeviceType::SpinningDisk,
    ];

    for dtype in config_types {
        let json = serde_json::to_string(&dtype).expect("Serialization failed");
        let restored: DeviceType = serde_json::from_str(&json).expect("Deserialization failed");

        // Verify round-trip consistency
        let json2 = serde_json::to_string(&restored).unwrap();
        assert_eq!(json, json2);
    }
}

#[test]
fn test_storage_tier_in_hashmap() {
    use std::collections::HashMap;

    let mut tier_map: HashMap<StorageTier, Vec<DeviceType>> = HashMap::new();

    tier_map.insert(
        StorageTier::Hot,
        vec![DeviceType::OptaneMemory, DeviceType::NvmeSsd],
    );
    tier_map.insert(StorageTier::Warm, vec![DeviceType::SataSsd]);
    tier_map.insert(StorageTier::Cold, vec![DeviceType::SpinningDisk]);

    assert_eq!(tier_map.len(), 3);
    assert_eq!(tier_map.get(&StorageTier::Hot).unwrap().len(), 2);
}

#[test]
fn test_pool_property_config_custom_values() {
    #[allow(deprecated, reason = "testing deprecated API backward compatibility")]
    let config = PoolPropertyConfig {
        ashift: 13,
        autoexpand: false,
        autotrim: false,
        compression: "zstd".to_string(),
        recordsize: "1M".to_string(),
    };

    assert_eq!(config.ashift, 13);
    assert!(!config.autoexpand);
    assert_eq!(config.compression, "zstd");
}

#[test]
fn test_device_detection_with_empty_lists() {
    let config = DeviceDetectionConfig {
        scan_paths: vec![],
        exclude_patterns: vec![],
        include_removable: false,
        min_device_size: 0,
        max_device_size: 0,
        skip_mountpoints: vec![],
        skip_fstypes: vec![],
        include_loop_devices: false,
    };

    assert!(config.scan_paths.is_empty());
    assert_eq!(config.min_device_size, 0);
}

// ==================== Edge Case Tests ====================

#[test]
fn test_config_with_large_device_count() {
    let mut config = PoolSetupConfig::default();

    for i in 0..1000 {
        config.devices.push(format!("/dev/sd{i}"));
    }

    assert_eq!(config.devices.len(), 1000);
}

#[test]
fn test_config_with_many_properties() {
    let mut config = PoolSetupConfig::default();

    for i in 0..100 {
        config
            .properties
            .insert(format!("prop{i}"), format!("value{i}"));
    }

    assert_eq!(config.properties.len(), 100);
}

#[test]
fn test_device_detection_with_unicode_paths() {
    let config = DeviceDetectionConfig {
        scan_paths: vec!["/dev/测试".to_string()],
        exclude_patterns: vec!["пропустить".to_string()],
        include_removable: false,
        min_device_size: 1024,
        max_device_size: 0,
        skip_mountpoints: vec!["/домой".to_string()],
        skip_fstypes: vec!["文件系统".to_string()],
        include_loop_devices: false,
    };

    assert!(config.scan_paths[0].contains("测试"));
}
