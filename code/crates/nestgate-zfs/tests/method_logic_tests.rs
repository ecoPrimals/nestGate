// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

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

//! High-value method tests that exercise business logic
//!
//! Focus: Methods that actually DO things, not just hold data
//! Target: Real coverage increase by testing logic paths

use nestgate_zfs::advanced_features::cache::*;
use nestgate_zfs::advanced_features::compression::*;
use nestgate_zfs::advanced_features::snapshots::*;
use nestgate_zfs::types::RetentionPolicy;

// ==================== CACHE ANALYTICS METHODS ====================

#[test]
fn test_arc_stats_collect_returns_valid_data() {
    let result = ArcStats::collect();

    assert!(result.is_ok(), "collect() should succeed");
    let stats = result.unwrap();

    // Verify business logic: hit_ratio + miss_ratio should equal 1.0
    let sum = stats.hit_ratio + stats.miss_ratio;
    assert!((sum - 1.0).abs() < 0.01, "Ratios should sum to ~1.0");

    // Verify sensible values
    assert!(stats.size > 0, "ARC size should be positive");
    assert!(stats.hit_ratio >= 0.0 && stats.hit_ratio <= 1.0);
    assert!(stats.miss_ratio >= 0.0 && stats.miss_ratio <= 1.0);
}

#[test]
fn test_l2arc_stats_collect_returns_valid_data() {
    let result = L2arcStats::collect();

    assert!(result.is_ok(), "collect() should succeed");
    let stats = result.unwrap();

    // Verify business logic
    assert!(stats.size > 0);
    let sum = stats.hit_ratio + stats.miss_ratio;
    assert!((sum - 1.0).abs() < 0.01);
}

#[test]
fn test_cache_efficiency_calculate_computes_correctly() {
    let arc_stats = ArcStats {
        size: 1024,
        hit_ratio: 0.90,
        miss_ratio: 0.10,
    };

    let l2arc_stats = L2arcStats {
        size: 2048,
        hit_ratio: 0.70,
        miss_ratio: 0.30,
    };

    // This actually executes the calculate logic
    let efficiency = CacheEfficiency::calculate(&arc_stats, &l2arc_stats);

    // Verify the calculation logic works
    assert!(efficiency.overall_efficiency > 0.0);
    assert!(efficiency.arc_efficiency > 0.0);
    assert!(efficiency.l2arc_efficiency > 0.0);

    // Verify relationships
    assert!(efficiency.arc_efficiency >= arc_stats.hit_ratio);
}

#[test]
fn test_cache_analytics_analyze_cache_performance() {
    let result = CacheAnalytics::analyze_cache_performance("test_pool");

    // This is testing the full method logic path
    assert!(result.is_ok(), "analyze_cache_performance should work");

    let analytics = result.unwrap();

    // Verify the method actually collected and calculated data
    assert!(analytics.arc_stats.size > 0);
    assert!(analytics.l2arc_stats.size > 0);
    assert!(analytics.efficiency.overall_efficiency >= 0.0);
}

// ==================== COMPRESSION ANALYTICS METHODS ====================

#[test]
fn test_compression_analyze_with_small_sample() {
    let data = vec![1u8; 100]; // 100 bytes
    let result = CompressionAnalytics::analyze_compression("tank/small", &data);

    assert!(result.is_ok());
    let analytics = result.unwrap();

    // Verify logic executed
    assert!(analytics.compression_ratio > 0.0);
    assert!(analytics.efficiency >= 0.0 && analytics.efficiency <= 100.0);
    assert!(!analytics.algorithm.is_empty());
}

#[test]
fn test_compression_analyze_with_large_sample() {
    let data = vec![2u8; 10_000]; // 10KB
    let result = CompressionAnalytics::analyze_compression("tank/large", &data);

    assert!(result.is_ok());
    let analytics = result.unwrap();

    // Same data should compress similarly regardless of size
    assert!(analytics.compression_ratio > 1.0);
}

#[test]
fn test_compression_analyze_with_empty_data() {
    let data = vec![]; // Empty
    let result = CompressionAnalytics::analyze_compression("tank/empty", &data);

    // Should handle empty data gracefully
    assert!(result.is_ok());
}

#[test]
fn test_compression_recommendations_logic_low_ratio() {
    let analytics = CompressionAnalytics {
        compression_ratio: 1.1, // Below 1.2 threshold
        efficiency: 9.1,
        algorithm: "lz4".to_string(),
    };

    // Test recommendation logic
    let recommendations = analytics.get_compression_recommendations();

    assert!(!recommendations.is_empty());
    // Should recommend disabling compression for low ratios
    assert!(recommendations.iter().any(|r| r.contains("disabling")));
}

#[test]
fn test_compression_recommendations_logic_medium_ratio() {
    let analytics = CompressionAnalytics {
        compression_ratio: 1.3, // Between 1.2 and 1.5
        efficiency: 23.1,
        algorithm: "lz4".to_string(),
    };

    let recommendations = analytics.get_compression_recommendations();

    assert!(!recommendations.is_empty());
    // Should recommend lz4 for medium ratios
    assert!(recommendations.iter().any(|r| r.contains("lz4")));
}

#[test]
fn test_compression_recommendations_logic_high_ratio() {
    let analytics = CompressionAnalytics {
        compression_ratio: 2.0, // Above 1.5 threshold
        efficiency: 50.0,
        algorithm: "lz4".to_string(),
    };

    let recommendations = analytics.get_compression_recommendations();

    assert!(!recommendations.is_empty());
    // Should recommend gzip for high compressibility
    assert!(recommendations.iter().any(|r| r.contains("gzip")));
}

// ==================== SNAPSHOT ANALYTICS METHODS ====================

#[test]
fn test_snapshot_analyze_with_few_snapshots() {
    let snapshots: Vec<String> = (0..10).map(|i| format!("snap_{}", i)).collect();
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

    // Verify the analysis logic ran
    assert_eq!(analytics.snapshot_count, 10);
    assert!(analytics.storage_usage > 0);
    // Few snapshots shouldn't trigger cleanup recommendation
    assert!(
        !analytics
            .recommendations
            .iter()
            .any(|r| r.contains("cleaning up"))
    );
}

#[test]
fn test_snapshot_analyze_with_many_snapshots() {
    let snapshots: Vec<String> = (0..100).map(|i| format!("snap_{}", i)).collect();
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

    assert_eq!(analytics.snapshot_count, 100);
    // Many snapshots (>50) should trigger cleanup recommendation
    assert!(
        analytics
            .recommendations
            .iter()
            .any(|r| r.contains("cleaning up") || r.contains("old snapshots"))
    );
}

#[test]
fn test_snapshot_analyze_high_retention_policy() {
    let snapshots: Vec<String> = vec!["snap1".to_string()];
    let policy = RetentionPolicy {
        name: "aggressive".to_string(),
        keep_hourly: 24,
        keep_daily: 100, // Very high - over 30 threshold
        keep_weekly: 52,
        keep_monthly: 120,
    };

    let result = SnapshotAnalytics::analyze_snapshots("tank/data", &snapshots, &policy);

    assert!(result.is_ok());
    let analytics = result.unwrap();

    // Should detect high daily retention
    assert!(
        analytics
            .recommendations
            .iter()
            .any(|r| r.contains("Daily") || r.contains("retention"))
    );
}

#[test]
fn test_snapshot_analyze_high_storage_usage() {
    // Create enough snapshots to exceed 10GB threshold
    let snapshots: Vec<String> = (0..200).map(|i| format!("snap_{}", i)).collect();
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

    // With 200 snapshots @ 100MB each = 20GB, should trigger storage warning
    assert!(analytics.storage_usage > 10 * 1024 * 1024 * 1024);
    assert!(
        analytics
            .recommendations
            .iter()
            .any(|r| r.contains("storage") || r.contains("significant"))
    );
}

#[test]
fn test_snapshot_analyze_empty_snapshots() {
    let snapshots: Vec<String> = vec![];
    let policy = RetentionPolicy {
        name: "minimal".to_string(),
        keep_hourly: 1,
        keep_daily: 1,
        keep_weekly: 0,
        keep_monthly: 0,
    };

    let result = SnapshotAnalytics::analyze_snapshots("tank/new", &snapshots, &policy);

    assert!(result.is_ok());
    let analytics = result.unwrap();

    assert_eq!(analytics.snapshot_count, 0);
    assert_eq!(analytics.storage_usage, 0);
}

// ==================== CACHE EFFICIENCY EDGE CASES ====================

#[test]
fn test_cache_efficiency_with_zero_hit_ratios() {
    let arc_stats = ArcStats {
        size: 1024,
        hit_ratio: 0.0,
        miss_ratio: 1.0,
    };

    let l2arc_stats = L2arcStats {
        size: 2048,
        hit_ratio: 0.0,
        miss_ratio: 1.0,
    };

    let efficiency = CacheEfficiency::calculate(&arc_stats, &l2arc_stats);

    // Should handle zero hit ratios without panic
    assert!(efficiency.overall_efficiency >= 0.0);
}

#[test]
fn test_cache_efficiency_with_perfect_hit_ratios() {
    let arc_stats = ArcStats {
        size: 1024,
        hit_ratio: 1.0,
        miss_ratio: 0.0,
    };

    let l2arc_stats = L2arcStats {
        size: 2048,
        hit_ratio: 1.0,
        miss_ratio: 0.0,
    };

    let efficiency = CacheEfficiency::calculate(&arc_stats, &l2arc_stats);

    // Perfect hits should result in high efficiency
    assert!(efficiency.overall_efficiency > 0.0);
    assert_eq!(efficiency.arc_efficiency, 1.0);
}

// ==================== INTEGRATION-STYLE TESTS ====================

#[test]
fn test_full_cache_analysis_pipeline() {
    // Test the complete flow: collect → analyze → efficiency
    let arc_result = ArcStats::collect();
    assert!(arc_result.is_ok());

    let l2arc_result = L2arcStats::collect();
    assert!(l2arc_result.is_ok());

    let arc_stats = arc_result.unwrap();
    let l2arc_stats = l2arc_result.unwrap();

    // Calculate efficiency from collected stats
    let efficiency = CacheEfficiency::calculate(&arc_stats, &l2arc_stats);

    // Verify the pipeline produces valid results
    assert!(efficiency.overall_efficiency >= 0.0);

    // Build full analytics
    let analytics = CacheAnalytics {
        arc_stats,
        l2arc_stats,
        efficiency,
    };

    assert!(analytics.arc_stats.size > 0);
    assert!(analytics.l2arc_stats.size > 0);
}

#[test]
fn test_full_compression_analysis_pipeline() {
    // Test the complete flow: analyze → recommendations
    let data = vec![42u8; 5000];
    let result = CompressionAnalytics::analyze_compression("tank/test", &data);

    assert!(result.is_ok());
    let analytics = result.unwrap();

    // Get recommendations based on analysis
    let recommendations = analytics.get_compression_recommendations();

    // Verify pipeline produces actionable output
    assert!(!recommendations.is_empty());
    assert!(analytics.compression_ratio > 0.0);
}

#[test]
fn test_full_snapshot_analysis_pipeline() {
    // Test the complete flow: snapshots → policy → analysis → recommendations
    let snapshots: Vec<String> = (0..75).map(|i| format!("auto-{}", i)).collect();

    let policy = RetentionPolicy {
        name: "production".to_string(),
        keep_hourly: 24,
        keep_daily: 14,
        keep_weekly: 8,
        keep_monthly: 24,
    };

    let result = SnapshotAnalytics::analyze_snapshots("tank/prod", &snapshots, &policy);

    assert!(result.is_ok());
    let analytics = result.unwrap();

    // Verify full pipeline execution
    assert_eq!(analytics.snapshot_count, 75);
    assert!(analytics.storage_usage > 0);
    // 75 snapshots should trigger recommendations
    assert!(!analytics.recommendations.is_empty());
}
