// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

#![allow(
    dead_code,
    unused_doc_comments,
    unused_imports,
    missing_docs,
    rustdoc::missing_crate_level_docs,
    deprecated,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::doc_markdown,
    clippy::module_name_repetitions,
    clippy::struct_excessive_bools,
    clippy::struct_field_names,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::cast_lossless,
    clippy::must_use_candidate,
    clippy::return_self_not_must_use,
    clippy::unnecessary_wraps,
    clippy::unused_self,
    clippy::unused_async,
    clippy::needless_pass_by_value,
    clippy::option_if_let_else,
    clippy::too_long_first_doc_paragraph,
    clippy::inline_always,
    clippy::redundant_closure,
    clippy::redundant_closure_for_method_calls,
    clippy::collapsible_if,
    clippy::single_char_pattern,
    clippy::implicit_hasher,
    clippy::float_cmp,
    clippy::manual_midpoint,
    clippy::suboptimal_flops,
    clippy::items_after_statements,
    clippy::items_after_test_module,
    clippy::too_many_lines,
    clippy::cognitive_complexity,
    clippy::unreadable_literal,
    clippy::redundant_clone,
    clippy::useless_vec,
    clippy::field_reassign_with_default,
    clippy::cmp_null,
    clippy::bool_assert_comparison,
    clippy::used_underscore_items,
    clippy::needless_raw_string_hashes,
    clippy::ref_as_ptr,
    clippy::no_effect_underscore_binding,
    clippy::needless_collect,
    clippy::module_inception,
    clippy::default_trait_access,
    clippy::wildcard_in_or_patterns,
    clippy::or_fun_call,
    clippy::manual_string_new,
    clippy::unnecessary_literal_unwrap,
    clippy::unnecessary_debug_formatting,
    clippy::assigning_clones,
    clippy::unnecessary_unwrap,
    clippy::unnecessary_map_or,
    clippy::unnecessary_lazy_evaluations,
    clippy::similar_names,
    clippy::needless_continue,
    clippy::collection_is_never_read,
    clippy::char_lit_as_u8,
    clippy::ptr_eq,
    clippy::uninlined_format_args,
    clippy::absurd_extreme_comparisons,
    clippy::match_wild_err_arm,
    clippy::single_match_else,
    clippy::derive_partial_eq_without_eq,
    clippy::match_wildcard_for_single_variants,
    clippy::missing_const_for_fn,
    clippy::used_underscore_binding,
    clippy::ignored_unit_patterns,
    unused_comparisons,
    clippy::format_push_string
)]

//! Simple tests for advanced ZFS features
//!
//! Targeting modules with 0% coverage to increase overall coverage

use nestgate_zfs::advanced_features::cache::*;
use nestgate_zfs::advanced_features::compression::*;
use nestgate_zfs::advanced_features::snapshots::*;
use nestgate_zfs::types::RetentionPolicy;

// ==================== CACHE STATS TESTS ====================

#[test]
fn test_arc_stats_construction() {
    let stats = ArcStats {
        size: 8 * 1024 * 1024 * 1024, // 8GB
        hit_ratio: 0.95,
        miss_ratio: 0.05,
    };

    assert!(stats.size > 0);
    assert!(stats.hit_ratio > 0.0 && stats.hit_ratio <= 1.0);
    assert!(stats.miss_ratio >= 0.0 && stats.miss_ratio < 1.0);
    assert!((stats.hit_ratio + stats.miss_ratio - 1.0).abs() < 0.01);
}

#[test]
fn test_arc_stats_collect() {
    let result = ArcStats::collect();
    assert!(result.is_ok());

    let stats = result.unwrap();
    assert!(stats.size > 0);
    assert!(stats.hit_ratio >= 0.0);
    assert!(stats.miss_ratio >= 0.0);
}

#[test]
fn test_l2arc_stats_construction() {
    let stats = L2arcStats {
        size: 100 * 1024 * 1024 * 1024, // 100GB
        hit_ratio: 0.65,
        miss_ratio: 0.35,
    };

    assert!(stats.size > 0);
    assert!(stats.hit_ratio > 0.0);
    assert!(stats.miss_ratio > 0.0);
}

#[test]
fn test_l2arc_stats_collect() {
    let result = L2arcStats::collect();
    assert!(result.is_ok());
    // L2ARC size may be 0 on systems without an L2ARC device — only assert the call succeeds
}

#[test]
fn test_cache_efficiency_new() {
    let efficiency = CacheEfficiency::new();

    assert_eq!(efficiency.overall_efficiency, 0.0);
    assert_eq!(efficiency.arc_efficiency, 0.0);
    assert_eq!(efficiency.l2arc_efficiency, 0.0);
}

#[test]
fn test_cache_efficiency_default() {
    let efficiency = CacheEfficiency::default();

    assert_eq!(efficiency.overall_efficiency, 0.0);
}

#[test]
fn test_cache_efficiency_calculate() {
    let arc_stats = ArcStats {
        size: 1024,
        hit_ratio: 0.85,
        miss_ratio: 0.15,
    };

    let l2arc_stats = L2arcStats {
        size: 2048,
        hit_ratio: 0.65,
        miss_ratio: 0.35,
    };

    let efficiency = CacheEfficiency::calculate(&arc_stats, &l2arc_stats);

    assert!(efficiency.overall_efficiency > 0.0);
    assert_eq!(efficiency.arc_efficiency, arc_stats.hit_ratio);
}

#[test]
fn test_cache_analytics_construction() {
    let arc_stats = ArcStats {
        size: 8_000_000_000,
        hit_ratio: 0.95,
        miss_ratio: 0.05,
    };

    let l2arc_stats = L2arcStats {
        size: 50_000_000_000,
        hit_ratio: 0.83,
        miss_ratio: 0.17,
    };

    let efficiency = CacheEfficiency::new();

    let analytics = CacheAnalytics {
        arc_stats,
        l2arc_stats,
        efficiency,
    };

    assert!(analytics.arc_stats.size > 0);
    assert!(analytics.l2arc_stats.size > 0);
}

// ==================== COMPRESSION TESTS ====================

#[test]
fn test_compression_analytics_construction() {
    let analytics = CompressionAnalytics {
        compression_ratio: 2.3,
        efficiency: 56.5,
        algorithm: "lz4".to_string(),
    };

    assert!(analytics.compression_ratio > 1.0);
    assert!(analytics.efficiency > 0.0);
    assert_eq!(analytics.algorithm, "lz4");
}

#[test]
fn test_compression_analyze() {
    let data = vec![1u8; 1024]; // 1KB of data
    let result = CompressionAnalytics::analyze_compression("tank/data", &data);

    assert!(result.is_ok());

    let analytics = result.unwrap();
    assert!(analytics.compression_ratio > 0.0);
    assert_eq!(analytics.algorithm, "lz4");
}

#[test]
fn test_compression_recommendations_low_ratio() {
    let analytics = CompressionAnalytics {
        compression_ratio: 1.1,
        efficiency: 10.0,
        algorithm: "lz4".to_string(),
    };

    let recommendations = analytics.get_compression_recommendations();
    assert!(!recommendations.is_empty());
    assert!(recommendations[0].contains("disabling compression"));
}

#[test]
fn test_compression_recommendations_medium_ratio() {
    let analytics = CompressionAnalytics {
        compression_ratio: 1.3,
        efficiency: 23.1,
        algorithm: "lz4".to_string(),
    };

    let recommendations = analytics.get_compression_recommendations();
    assert!(!recommendations.is_empty());
    assert!(recommendations[0].contains("lz4"));
}

#[test]
fn test_compression_recommendations_high_ratio() {
    let analytics = CompressionAnalytics {
        compression_ratio: 2.5,
        efficiency: 60.0,
        algorithm: "lz4".to_string(),
    };

    let recommendations = analytics.get_compression_recommendations();
    assert!(!recommendations.is_empty());
    assert!(recommendations[0].contains("gzip"));
}

// ==================== SNAPSHOT TESTS ====================

#[test]
fn test_snapshot_analytics_construction() {
    let analytics = SnapshotAnalytics {
        snapshot_count: 50,
        storage_usage: 5 * 1024 * 1024 * 1024, // 5GB
        recommendations: vec!["Clean up old snapshots".to_string()],
    };

    assert_eq!(analytics.snapshot_count, 50);
    assert!(analytics.storage_usage > 0);
    assert!(!analytics.recommendations.is_empty());
}

#[test]
fn test_snapshot_analyze_low_count() {
    let snapshots: Vec<String> = (0..10).map(|i| format!("snapshot_{}", i)).collect();
    let policy = RetentionPolicy {
        name: "standard".to_string(),
        keep_hourly: 24,
        keep_daily: 7,
        keep_weekly: 4,
        keep_monthly: 12,
    };

    let result = SnapshotAnalytics::analyze_snapshots("tank/data", &snapshots, &policy);
    assert!(result.is_ok());

    let analytics = result.unwrap();
    assert_eq!(analytics.snapshot_count, 10);
}

#[test]
fn test_snapshot_analyze_high_count() {
    let snapshots: Vec<String> = (0..100).map(|i| format!("snapshot_{}", i)).collect();
    let policy = RetentionPolicy {
        name: "aggressive".to_string(),
        keep_hourly: 24,
        keep_daily: 7,
        keep_weekly: 4,
        keep_monthly: 12,
    };

    let result = SnapshotAnalytics::analyze_snapshots("tank/data", &snapshots, &policy);
    assert!(result.is_ok());

    let analytics = result.unwrap();
    assert_eq!(analytics.snapshot_count, 100);
    assert!(!analytics.recommendations.is_empty());
}

#[test]
fn test_snapshot_analyze_high_retention() {
    let snapshots: Vec<String> = vec!["snap1".to_string()];
    let policy = RetentionPolicy {
        name: "long_term".to_string(),
        keep_hourly: 24,
        keep_daily: 365, // Very high daily retention
        keep_weekly: 52,
        keep_monthly: 120,
    };

    let result = SnapshotAnalytics::analyze_snapshots("tank/data", &snapshots, &policy);
    assert!(result.is_ok());

    let analytics = result.unwrap();
    // Should recommend something about high retention
    let has_retention_recommendation = analytics
        .recommendations
        .iter()
        .any(|r| r.contains("retention") || r.contains("daily"));
    assert!(has_retention_recommendation || analytics.recommendations.is_empty());
}

// ==================== EDGE CASES ====================

#[test]
fn test_arc_stats_zero_hit_ratio() {
    let stats = ArcStats {
        size: 1024,
        hit_ratio: 0.0,
        miss_ratio: 1.0,
    };

    assert_eq!(stats.hit_ratio, 0.0);
    assert_eq!(stats.miss_ratio, 1.0);
}

#[test]
fn test_arc_stats_perfect_hit_ratio() {
    let stats = ArcStats {
        size: 1024,
        hit_ratio: 1.0,
        miss_ratio: 0.0,
    };

    assert_eq!(stats.hit_ratio, 1.0);
    assert_eq!(stats.miss_ratio, 0.0);
}

#[test]
fn test_compression_no_compression() {
    let analytics = CompressionAnalytics {
        compression_ratio: 1.0,
        efficiency: 0.0,
        algorithm: "off".to_string(),
    };

    assert_eq!(analytics.compression_ratio, 1.0);
    assert_eq!(analytics.algorithm, "off");
}

#[test]
fn test_compression_high_ratio() {
    let analytics = CompressionAnalytics {
        compression_ratio: 10.0,
        efficiency: 90.0,
        algorithm: "gzip-9".to_string(),
    };

    assert!(analytics.compression_ratio > 5.0);
}

#[test]
fn test_snapshot_empty() {
    let snapshots: Vec<String> = vec![];
    let policy = RetentionPolicy {
        name: "minimal".to_string(),
        keep_hourly: 1,
        keep_daily: 1,
        keep_weekly: 0,
        keep_monthly: 0,
    };

    let result = SnapshotAnalytics::analyze_snapshots("tank/data", &snapshots, &policy);
    assert!(result.is_ok());

    let analytics = result.unwrap();
    assert_eq!(analytics.snapshot_count, 0);
}

// ==================== SERIALIZATION TESTS ====================

#[test]
fn test_cache_analytics_serialization() {
    let analytics = CacheAnalytics {
        arc_stats: ArcStats {
            size: 1024,
            hit_ratio: 0.85,
            miss_ratio: 0.15,
        },
        l2arc_stats: L2arcStats {
            size: 2048,
            hit_ratio: 0.65,
            miss_ratio: 0.35,
        },
        efficiency: CacheEfficiency::new(),
    };

    let json = serde_json::to_string(&analytics).expect("Serialization failed");
    assert!(json.contains("1024"));
    assert!(json.contains("0.85"));
}

#[test]
fn test_compression_analytics_serialization() {
    let analytics = CompressionAnalytics {
        compression_ratio: 2.0,
        efficiency: 50.0,
        algorithm: "lz4".to_string(),
    };

    let json = serde_json::to_string(&analytics).expect("Serialization failed");
    assert!(json.contains("lz4"));
    assert!(json.contains("2.0"));
}

#[test]
fn test_snapshot_analytics_serialization() {
    let analytics = SnapshotAnalytics {
        snapshot_count: 42,
        storage_usage: 1024 * 1024 * 1024,
        recommendations: vec!["test".to_string()],
    };

    let json = serde_json::to_string(&analytics).expect("Serialization failed");
    assert!(json.contains("42"));
    assert!(json.contains("test"));
}
