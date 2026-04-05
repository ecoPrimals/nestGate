// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::float_cmp
)]

use super::*;
use std::collections::HashMap;
use std::time::Duration;

// ==================== StorageTier Tests ====================

#[test]
fn test_storage_tier_priority() {
    assert_eq!(StorageTier::Hot.priority(), 0);
    assert_eq!(StorageTier::Warm.priority(), 1);
    assert_eq!(StorageTier::Cool.priority(), 2);
    assert_eq!(StorageTier::Cold.priority(), 3);
    assert_eq!(StorageTier::Frozen.priority(), 4);
}

#[test]
fn test_storage_tier_priority_ordering() {
    assert!(StorageTier::Hot.priority() < StorageTier::Warm.priority());
    assert!(StorageTier::Warm.priority() < StorageTier::Cool.priority());
    assert!(StorageTier::Cool.priority() < StorageTier::Cold.priority());
    assert!(StorageTier::Cold.priority() < StorageTier::Frozen.priority());
}

#[test]
fn test_storage_tier_typical_access_time() {
    assert_eq!(
        StorageTier::Hot.typical_access_time(),
        Duration::from_micros(100)
    );
    assert_eq!(
        StorageTier::Warm.typical_access_time(),
        Duration::from_millis(1)
    );
    assert_eq!(
        StorageTier::Cool.typical_access_time(),
        Duration::from_millis(10)
    );
    assert_eq!(
        StorageTier::Cold.typical_access_time(),
        Duration::from_millis(100)
    );
    assert_eq!(
        StorageTier::Frozen.typical_access_time(),
        Duration::from_secs(10)
    );
}

#[test]
fn test_storage_tier_access_time_ordering() {
    assert!(StorageTier::Hot.typical_access_time() < StorageTier::Warm.typical_access_time());
    assert!(StorageTier::Warm.typical_access_time() < StorageTier::Cool.typical_access_time());
    assert!(StorageTier::Cool.typical_access_time() < StorageTier::Cold.typical_access_time());
    assert!(StorageTier::Cold.typical_access_time() < StorageTier::Frozen.typical_access_time());
}

#[test]
fn test_storage_tier_custom_ext() {
    let custom = StorageTier::Custom("edge-cache".to_string());
    assert_eq!(custom.priority(), 5);
    assert_eq!(custom.typical_access_time(), Duration::from_millis(50));
}

#[test]
fn test_storage_tier_default_is_hot() {
    assert_eq!(StorageTier::default(), StorageTier::Hot);
}

// ==================== CachePolicy Tests ====================

#[test]
fn test_cache_policy_display() {
    assert_eq!(CachePolicy::None.to_string(), "none");
    assert_eq!(CachePolicy::ReadOnly.to_string(), "read-only");
    assert_eq!(CachePolicy::WriteThrough.to_string(), "write-through");
    assert_eq!(CachePolicy::WriteBack.to_string(), "write-back");
}

#[test]
fn test_cache_policy_default() {
    assert_eq!(CachePolicy::default(), CachePolicy::WriteThrough);
}

#[test]
fn test_cache_policy_equality() {
    assert_eq!(CachePolicy::None, CachePolicy::None);
    assert_ne!(CachePolicy::None, CachePolicy::ReadOnly);
}

#[test]
fn test_cache_policy_serde_roundtrip() {
    for policy in [
        CachePolicy::None,
        CachePolicy::ReadOnly,
        CachePolicy::WriteThrough,
        CachePolicy::WriteBack,
    ] {
        let json = serde_json::to_string(&policy).expect("serialize CachePolicy");
        let back: CachePolicy = serde_json::from_str(&json).expect("deserialize CachePolicy");
        assert_eq!(policy, back);
    }
}

// ==================== CacheStats Tests ====================

#[test]
fn test_cache_stats_default() {
    let stats = CacheStats::default();
    assert_eq!(stats.hits, 0);
    assert_eq!(stats.misses, 0);
    assert_eq!(stats.hot_tier_items, 0);
    assert_eq!(stats.total_items(), 0);
    assert_eq!(stats.total_size_bytes(), 0);
    assert_eq!(stats.total_evictions(), 0);
}

#[test]
fn test_cache_stats_hit_ratio_zero_operations() {
    let stats = CacheStats::default();
    assert_eq!(stats.hit_ratio(), 0.0);
}

#[test]
fn test_cache_stats_hit_ratio_all_hits() {
    let stats = CacheStats {
        hits: 10,
        misses: 0,
        ..Default::default()
    };
    assert_eq!(stats.hit_ratio(), 1.0);
}

#[test]
fn test_cache_stats_hit_ratio_all_misses() {
    let stats = CacheStats {
        hits: 0,
        misses: 10,
        ..Default::default()
    };
    assert_eq!(stats.hit_ratio(), 0.0);
}

#[test]
fn test_cache_stats_hit_ratio_mixed() {
    let stats = CacheStats {
        hits: 7,
        misses: 3,
        ..Default::default()
    };
    assert!((stats.hit_ratio() - 0.7).abs() < 0.001);
}

#[test]
fn test_cache_stats_total_items() {
    let stats = CacheStats {
        hot_tier_items: 10,
        warm_tier_items: 20,
        cold_tier_items: 30,
        ..Default::default()
    };
    assert_eq!(stats.total_items(), 60);
}

#[test]
fn test_cache_stats_total_size_bytes() {
    let stats = CacheStats {
        hot_tier_size_bytes: 1000,
        warm_tier_size_bytes: 2000,
        cold_tier_size_bytes: 3000,
        ..Default::default()
    };
    assert_eq!(stats.total_size_bytes(), 6000);
}

#[test]
fn test_cache_stats_total_evictions() {
    let stats = CacheStats {
        hot_tier_evictions: 5,
        warm_tier_evictions: 10,
        cold_tier_evictions: 15,
        ..Default::default()
    };
    assert_eq!(stats.total_evictions(), 30);
}

#[test]
fn test_cache_stats_record_hit() {
    let mut stats = CacheStats::default();
    assert_eq!(stats.hits, 0);
    stats.record_hit();
    assert_eq!(stats.hits, 1);
    stats.record_hit();
    assert_eq!(stats.hits, 2);
}

#[test]
fn test_cache_stats_record_miss() {
    let mut stats = CacheStats::default();
    assert_eq!(stats.misses, 0);
    stats.record_miss();
    assert_eq!(stats.misses, 1);
    stats.record_miss();
    assert_eq!(stats.misses, 2);
}

#[test]
fn test_cache_stats_record_access_time() {
    let mut stats = CacheStats::default();
    let access_time = Duration::from_millis(5);
    stats.record_access_time(StorageTier::Hot, access_time);

    let recorded_time = stats.tier_access_times.get(&StorageTier::Hot);
    assert!(recorded_time.is_some());
}

#[test]
fn test_cache_stats_record_access_time_average() {
    let mut stats = CacheStats::default();
    // Default Hot tier = 100µs. Running average: (old + new) / 2
    // After 10ms: (100µs + 10ms) / 2 = 5050µs
    stats.record_access_time(StorageTier::Hot, Duration::from_millis(10));
    // After 20ms: (5050µs + 20ms) / 2 = 12525µs
    stats.record_access_time(StorageTier::Hot, Duration::from_millis(20));

    let avg = stats
        .tier_access_times
        .get(&StorageTier::Hot)
        .expect("Should have access time");
    assert_eq!(*avg, Duration::from_micros(12525));
}

#[test]
fn test_cache_stats_record_access_time_tier_not_in_defaults() {
    let mut stats = CacheStats::default();
    assert!(!stats.tier_access_times.contains_key(&StorageTier::Cold));
    stats.record_access_time(StorageTier::Cold, Duration::from_millis(3));
    assert_eq!(
        *stats
            .tier_access_times
            .get(&StorageTier::Cold)
            .expect("cold tier inserted"),
        Duration::from_millis(3)
    );
}

#[test]
fn test_cache_stats_serde_roundtrip() {
    let stats = CacheStats {
        hits: 1,
        misses: 2,
        hot_tier_items: 3,
        warm_tier_items: 4,
        cold_tier_items: 5,
        hot_tier_size_bytes: 6,
        warm_tier_size_bytes: 7,
        cold_tier_size_bytes: 8,
        hot_tier_evictions: 9,
        warm_tier_evictions: 10,
        cold_tier_evictions: 11,
        tier_access_times: HashMap::from([(StorageTier::Hot, Duration::from_nanos(1))]),
        efficiency_metrics: EfficiencyMetrics::default(),
    };
    let json = serde_json::to_string(&stats).expect("serialize CacheStats");
    let back: CacheStats = serde_json::from_str(&json).expect("deserialize CacheStats");
    assert_eq!(stats.hits, back.hits);
    assert_eq!(stats.misses, back.misses);
    assert_eq!(stats.tier_access_times, back.tier_access_times);
    assert_eq!(
        stats.efficiency_metrics.moving_hit_ratio,
        back.efficiency_metrics.moving_hit_ratio
    );
}

// ==================== EfficiencyMetrics Tests ====================

#[test]
fn test_efficiency_metrics_default() {
    let metrics = EfficiencyMetrics::default();
    assert_eq!(metrics.moving_hit_ratio, 0.0);
    assert_eq!(metrics.peak_hit_ratio, 0.0);
    assert_eq!(metrics.effectiveness_score, 0.0);
}

#[test]
fn test_efficiency_metrics_update_hit() {
    let mut metrics = EfficiencyMetrics::default();
    metrics.update_hit();
    assert_eq!(metrics.moving_hit_ratio, 1.0);
    assert_eq!(metrics.peak_hit_ratio, 1.0);
}

#[test]
fn test_efficiency_metrics_update_miss() {
    let mut metrics = EfficiencyMetrics::default();
    metrics.update_miss();
    assert_eq!(metrics.moving_hit_ratio, 0.0);
}

#[test]
fn test_efficiency_metrics_mixed_operations() {
    let mut metrics = EfficiencyMetrics::default();
    metrics.update_hit();
    metrics.update_hit();
    metrics.update_miss();
    // 2 hits out of 3 operations = 0.666...
    assert!((metrics.moving_hit_ratio - 0.666).abs() < 0.01);
}

#[test]
fn test_efficiency_metrics_peak_hit_ratio() {
    let mut metrics = EfficiencyMetrics::default();
    metrics.update_hit();
    metrics.update_hit();
    assert_eq!(metrics.peak_hit_ratio, 1.0);

    metrics.update_miss();
    metrics.update_miss();
    // Peak should remain at 1.0 even though current ratio dropped
    assert_eq!(metrics.peak_hit_ratio, 1.0);
}

#[test]
fn test_efficiency_metrics_max_operations_tracking() {
    let mut metrics = EfficiencyMetrics::default();
    // Add more than max_operations_tracked (1000)
    for _ in 0..1100 {
        metrics.update_hit();
    }
    assert_eq!(metrics.last_operations_len(), 1000);
}

#[test]
fn test_efficiency_metrics_serde_roundtrip() {
    let mut metrics = EfficiencyMetrics::default();
    metrics.update_hit();
    metrics.update_miss();
    let json = serde_json::to_string(&metrics).expect("serialize EfficiencyMetrics");
    let back: EfficiencyMetrics = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(metrics.moving_hit_ratio, back.moving_hit_ratio);
    assert_eq!(metrics.peak_hit_ratio, back.peak_hit_ratio);
    assert_eq!(metrics.effectiveness_score, back.effectiveness_score);
}

#[test]
fn test_efficiency_metrics_consistency_bonus_at_100_operations() {
    let mut metrics = EfficiencyMetrics::default();
    for _ in 0..100 {
        metrics.update_hit();
    }
    assert_eq!(metrics.last_operations_len(), 100);
    assert!((metrics.effectiveness_score - 100.0).abs() < f64::EPSILON);
}

// ==================== CacheEntry Tests ====================

#[test]
fn test_cache_entry_new() {
    let key = "test-key".to_string();
    let data = vec![1, 2, 3, 4, 5];
    let entry = CacheEntry::new(key.clone(), data.clone(), StorageTier::Hot);

    assert_eq!(entry.key, key);
    assert_eq!(entry.data, data);
    assert_eq!(entry.size, 5);
    assert_eq!(entry.access_count, 0);
    assert_eq!(entry.tier, StorageTier::Hot);
    assert!(entry.ttl.is_none());
}

#[test]
fn test_cache_entry_new_empty_data() {
    let entry = CacheEntry::new("k".to_string(), Vec::new(), StorageTier::Warm);
    assert_eq!(entry.size, 0);
    assert!(entry.data.is_empty());
}

#[test]
fn test_cache_entry_is_expired_past_ttl() {
    let mut entry = CacheEntry::new("key".to_string(), vec![1], StorageTier::Hot);
    entry.created_at = chrono::DateTime::from_timestamp(0, 0).expect("epoch");
    entry.ttl = Some(Duration::from_secs(1));
    assert!(entry.is_expired());
}

#[test]
fn test_cache_entry_serde_roundtrip() {
    let entry = CacheEntry::new("k".to_string(), vec![0, 255], StorageTier::Cool);
    let json = serde_json::to_string(&entry).expect("serialize CacheEntry");
    let back: CacheEntry = serde_json::from_str(&json).expect("deserialize CacheEntry");
    assert_eq!(entry.key, back.key);
    assert_eq!(entry.data, back.data);
    assert_eq!(entry.size, back.size);
    assert_eq!(entry.tier, back.tier);
    assert_eq!(entry.ttl, back.ttl);
}

#[test]
fn test_cache_entry_is_expired_no_ttl() {
    let entry = CacheEntry::new("key".to_string(), vec![1, 2, 3], StorageTier::Hot);
    assert!(!entry.is_expired());
}

#[test]
fn test_cache_entry_is_expired_with_future_ttl() {
    let mut entry = CacheEntry::new("key".to_string(), vec![1, 2, 3], StorageTier::Hot);
    entry.ttl = Some(Duration::from_secs(3600)); // 1 hour
    assert!(!entry.is_expired());
}

#[tokio::test]
async fn test_cache_entry_touch() {
    let mut entry = CacheEntry::new("key".to_string(), vec![1, 2, 3], StorageTier::Hot);
    let initial_access_count = entry.access_count;
    let initial_accessed_at = entry.accessed_at;

    // Sleep a tiny bit to ensure time difference (non-blocking, concurrent)
    tokio::time::sleep(Duration::from_millis(10)).await;

    entry.touch();

    assert_eq!(entry.access_count, initial_access_count + 1);
    assert!(entry.accessed_at > initial_accessed_at);
}

#[test]
fn test_cache_entry_touch_multiple_times() {
    let mut entry = CacheEntry::new("key".to_string(), vec![1, 2, 3], StorageTier::Hot);

    entry.touch();
    entry.touch();
    entry.touch();

    assert_eq!(entry.access_count, 3);
}

#[test]
fn test_cache_entry_age() {
    let entry = CacheEntry::new("key".to_string(), vec![1, 2, 3], StorageTier::Hot);
    let age = entry.age();

    // Age should be very small (just created)
    assert!(age.num_milliseconds() < 1000);
}

#[test]
fn test_cache_entry_size_calculation() {
    let data = vec![0u8; 1024]; // 1 KB
    let entry = CacheEntry::new("key".to_string(), data, StorageTier::Hot);
    assert_eq!(entry.size, 1024);
}

#[test]
fn test_cache_entry_different_tiers() {
    let entry_hot = CacheEntry::new("key".to_string(), vec![1], StorageTier::Hot);
    let entry_warm = CacheEntry::new("key".to_string(), vec![1], StorageTier::Warm);
    let entry_cold = CacheEntry::new("key".to_string(), vec![1], StorageTier::Cold);

    assert_eq!(entry_hot.tier, StorageTier::Hot);
    assert_eq!(entry_warm.tier, StorageTier::Warm);
    assert_eq!(entry_cold.tier, StorageTier::Cold);
}
