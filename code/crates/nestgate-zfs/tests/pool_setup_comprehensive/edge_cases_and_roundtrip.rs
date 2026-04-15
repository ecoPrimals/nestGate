// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Boundary inputs for names and collections, plus serde round-trips for config/result.

use nestgate_zfs::pool_setup::{
    DeviceDetectionConfig, PoolSetupConfig, PoolSetupResult, PoolTopology, RedundancyLevel,
};
use std::collections::HashMap;

// ==================== EDGE CASE TESTS ====================

#[test]
fn test_empty_pool_name() {
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
