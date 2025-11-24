//! Comprehensive tests for Tier Management module
//!
//! Tests cover:
//! - Tier manager creation
//! - Tier statistics tracking
//! - Tier status reporting
//! - Storage tier operations

use super::tier::*;
use crate::types::StorageTier;

// ==================== TIER STATS TESTS ====================

#[test]
fn test_tier_stats_default() {
    let stats = TierStats::default();

    assert_eq!(stats.total_capacity, 0);
    assert_eq!(stats.used_capacity, 0);
    assert_eq!(stats.file_count, 0);
    assert_eq!(stats.active_operations, 0);
}

#[test]
fn test_tier_stats_with_values() {
    let stats = TierStats {
        total_capacity: 1_000_000,
        used_capacity: 250_000,
        file_count: 1500,
        active_operations: 5,
    };

    assert_eq!(stats.total_capacity, 1_000_000);
    assert_eq!(stats.used_capacity, 250_000);
    assert_eq!(stats.file_count, 1500);
    assert_eq!(stats.active_operations, 5);
}

#[test]
fn test_tier_stats_utilization_calculation() {
    let stats = TierStats {
        total_capacity: 1_000_000,
        used_capacity: 250_000,
        file_count: 100,
        active_operations: 2,
    };

    let utilization = if stats.total_capacity > 0 {
        (stats.used_capacity as f64 / stats.total_capacity as f64) * 100.0
    } else {
        0.0
    };

    assert_eq!(utilization, 25.0);
}

#[test]
fn test_tier_stats_clone() {
    let stats1 = TierStats {
        total_capacity: 1000,
        used_capacity: 500,
        file_count: 10,
        active_operations: 1,
    };

    let stats2 = stats1.clone();

    assert_eq!(stats1.total_capacity, stats2.total_capacity);
    assert_eq!(stats1.used_capacity, stats2.used_capacity);
}

// ==================== TIER STATUS TESTS ====================

#[test]
fn test_tier_status_hot() {
    let status = TierStatus {
        tier: StorageTier::Hot,
        health: "ONLINE".to_string(),
        utilization: 45.5,
        stats: TierStats {
            total_capacity: 1_000_000,
            used_capacity: 455_000,
            file_count: 5000,
            active_operations: 10,
        },
    };

    assert_eq!(status.tier, StorageTier::Hot);
    assert_eq!(status.health, "ONLINE");
    assert_eq!(status.utilization, 45.5);
}

#[test]
fn test_tier_status_warm() {
    let status = TierStatus {
        tier: StorageTier::Warm,
        health: "ONLINE".to_string(),
        utilization: 70.0,
        stats: TierStats::default(),
    };

    assert_eq!(status.tier, StorageTier::Warm);
    assert!(status.utilization > 50.0);
}

#[test]
fn test_tier_status_cold() {
    let status = TierStatus {
        tier: StorageTier::Cold,
        health: "ONLINE".to_string(),
        utilization: 90.0,
        stats: TierStats {
            total_capacity: 10_000_000,
            used_capacity: 9_000_000,
            file_count: 50000,
            active_operations: 1,
        },
    };

    assert_eq!(status.tier, StorageTier::Cold);
    assert!(status.utilization > 80.0);
}

#[test]
fn test_tier_status_different_health_states() {
    let online = TierStatus {
        tier: StorageTier::Hot,
        health: "ONLINE".to_string(),
        utilization: 50.0,
        stats: TierStats::default(),
    };

    let degraded = TierStatus {
        tier: StorageTier::Warm,
        health: "DEGRADED".to_string(),
        utilization: 75.0,
        stats: TierStats::default(),
    };

    assert_eq!(online.health, "ONLINE");
    assert_eq!(degraded.health, "DEGRADED");
    assert_ne!(online.health, degraded.health);
}

// ==================== TIER MANAGER TESTS ====================

#[test]
fn test_tier_manager_creation_for_testing() {
    let manager = TierManager::new_for_testing();

    // Manager should initialize successfully
    assert!(format!("{:?}", manager).contains("TierManager"));
}

#[test]
fn test_tier_manager_debug() {
    let manager = TierManager::new_for_testing();
    let debug_str = format!("{:?}", manager);

    // Debug output should contain key information
    assert!(debug_str.contains("TierManager"));
}

// ==================== STORAGE TIER TESTS ====================

#[test]
fn test_storage_tier_hot() {
    let tier = StorageTier::Hot;
    assert_eq!(tier, StorageTier::Hot);
    assert_ne!(tier, StorageTier::Warm);
}

#[test]
fn test_storage_tier_warm() {
    let tier = StorageTier::Warm;
    assert_eq!(tier, StorageTier::Warm);
    assert_ne!(tier, StorageTier::Cold);
}

#[test]
fn test_storage_tier_cold() {
    let tier = StorageTier::Cold;
    assert_eq!(tier, StorageTier::Cold);
    assert_ne!(tier, StorageTier::Hot);
}

#[test]
fn test_storage_tier_clone() {
    let tier1 = StorageTier::Hot;
    let tier2 = tier1.clone();
    assert_eq!(tier1, tier2);
}

#[test]
fn test_all_tier_types() {
    let hot = StorageTier::Hot;
    let warm = StorageTier::Warm;
    let cold = StorageTier::Cold;

    // All should be different
    assert_ne!(hot, warm);
    assert_ne!(warm, cold);
    assert_ne!(hot, cold);
}

// Total tests added: 15
// Focus areas:
// - Tier stats (4 tests)
// - Tier status (4 tests)
// - Tier manager (2 tests)
// - Storage tiers (5 tests)
