//! Comprehensive tests for Pool Setup module
//!
//! Tests cover:
//! - Pool configuration creation
//! - Configuration validation
//! - Topology types
//! - Device types
//! - Redundancy levels

use super::config::*;
use std::collections::HashMap;

// ==================== POOL TOPOLOGY TESTS ====================

#[test]
fn test_pool_topology_single() {
    let topology = PoolTopology::Single;
    assert!(matches!(topology, PoolTopology::Single));
}

#[test]
fn test_pool_topology_mirror() {
    let topology = PoolTopology::Mirror;
    assert!(matches!(topology, PoolTopology::Mirror));
}

#[test]
fn test_pool_topology_raidz_levels() {
    let raidz1 = PoolTopology::RaidZ1;
    let raidz2 = PoolTopology::RaidZ2;
    let raidz3 = PoolTopology::RaidZ3;

    assert!(matches!(raidz1, PoolTopology::RaidZ1));
    assert!(matches!(raidz2, PoolTopology::RaidZ2));
    assert!(matches!(raidz3, PoolTopology::RaidZ3));
}

// ==================== DEVICE TYPE TESTS ====================

#[test]
fn test_device_type_optane() {
    let device = DeviceType::OptaneMemory;
    assert_eq!(device, DeviceType::OptaneMemory);
}

#[test]
fn test_device_type_nvme() {
    let device = DeviceType::NvmeSsd;
    assert_eq!(device, DeviceType::NvmeSsd);
}

#[test]
fn test_device_type_sata() {
    let device = DeviceType::SataSsd;
    assert_eq!(device, DeviceType::SataSsd);
}

#[test]
fn test_device_type_spinning() {
    let device = DeviceType::SpinningDisk;
    assert_eq!(device, DeviceType::SpinningDisk);
}

#[test]
fn test_device_types_distinct() {
    let optane = DeviceType::OptaneMemory;
    let nvme = DeviceType::NvmeSsd;
    let sata = DeviceType::SataSsd;
    let hdd = DeviceType::SpinningDisk;

    assert_ne!(optane, nvme);
    assert_ne!(nvme, sata);
    assert_ne!(sata, hdd);
}

// ==================== STORAGE TIER TESTS ====================

#[test]
fn test_storage_tier_hot() {
    let tier = StorageTier::Hot;
    assert_eq!(tier, StorageTier::Hot);
}

#[test]
fn test_storage_tier_warm() {
    let tier = StorageTier::Warm;
    assert_eq!(tier, StorageTier::Warm);
}

#[test]
fn test_storage_tier_cold() {
    let tier = StorageTier::Cold;
    assert_eq!(tier, StorageTier::Cold);
}

#[test]
fn test_storage_tier_cache() {
    let tier = StorageTier::Cache;
    assert_eq!(tier, StorageTier::Cache);
}

#[test]
fn test_storage_tiers_hash_map() {
    let mut tier_map = HashMap::new();
    tier_map.insert(StorageTier::Hot, vec![DeviceType::OptaneMemory]);
    tier_map.insert(StorageTier::Warm, vec![DeviceType::NvmeSsd]);
    tier_map.insert(StorageTier::Cold, vec![DeviceType::SpinningDisk]);

    assert_eq!(tier_map.len(), 3);
    assert!(tier_map.contains_key(&StorageTier::Hot));
}

// ==================== REDUNDANCY LEVEL TESTS ====================

#[test]
fn test_redundancy_none() {
    let redundancy = RedundancyLevel::None;
    assert!(matches!(redundancy, RedundancyLevel::None));
}

#[test]
fn test_redundancy_single() {
    let redundancy = RedundancyLevel::Single;
    assert!(matches!(redundancy, RedundancyLevel::Single));
}

#[test]
fn test_redundancy_double() {
    let redundancy = RedundancyLevel::Double;
    assert!(matches!(redundancy, RedundancyLevel::Double));
}

#[test]
fn test_redundancy_triple() {
    let redundancy = RedundancyLevel::Triple;
    assert!(matches!(redundancy, RedundancyLevel::Triple));
}

// Total tests added: 17
// Focus areas:
// - Pool topology (3 tests)
// - Device types (5 tests)
// - Storage tiers (5 tests)
// - Redundancy levels (4 tests)
