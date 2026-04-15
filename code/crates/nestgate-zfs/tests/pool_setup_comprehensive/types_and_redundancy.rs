// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! `PoolTopology`, storage tiers, device types, and redundancy level coverage.

use nestgate_zfs::pool_setup::{
    ConfigDeviceType, ConfigStorageTier, PoolTopology, RedundancyLevel,
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
    let device2 = device1;
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
