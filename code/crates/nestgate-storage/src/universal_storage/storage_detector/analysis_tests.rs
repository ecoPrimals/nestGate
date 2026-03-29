// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Comprehensive tests for Storage Analyzer
//!
//! Tests cover storage analysis, scoring, recommendations, and optimization suggestions.

use super::{StorageAnalyzer, StorageUseCase};
use crate::universal_storage::storage_detector::{
    CostProfile, DetectedStorage, PerformanceProfile,
};
use nestgate_types::unified_enums::storage_types::{UnifiedStorageCapability, UnifiedStorageType};
use std::collections::HashMap;

// ==================== TEST HELPERS ====================

/// Configuration for creating test storage
#[derive(Clone)]
struct TestStorageConfig {
    name: String,
    storage_type: UnifiedStorageType,
    read_throughput: f64,
    read_latency: f64,
    capacity_gb: u64,
    cost_per_gb: f64,
    reliability: f64,
    capabilities: Vec<UnifiedStorageCapability>,
}

/// Create a test storage system with specified parameters
#[allow(clippy::too_many_arguments)]
fn create_test_storage(
    name: &str,
    storage_type: UnifiedStorageType,
    read_throughput: f64,
    read_latency: f64,
    capacity_gb: u64,
    cost_per_gb: f64,
    reliability: f64,
    capabilities: Vec<UnifiedStorageCapability>,
) -> DetectedStorage {
    let config = TestStorageConfig {
        name: name.to_string(),
        storage_type,
        read_throughput,
        read_latency,
        capacity_gb,
        cost_per_gb,
        reliability,
        capabilities,
    };
    create_test_storage_from_config(&config)
}

/// Create a test storage system from config struct
fn create_test_storage_from_config(config: &TestStorageConfig) -> DetectedStorage {
    DetectedStorage {
        identifier: format!("{}_id", config.name),
        storage_type: config.storage_type.clone(),
        display_name: config.name.clone(),
        capabilities: config.capabilities.clone(),
        performance_profile: PerformanceProfile {
            read_throughput_mbps: config.read_throughput,
            write_throughput_mbps: config.read_throughput * 0.8,
            read_latency_us: config.read_latency,
            write_latency_us: config.read_latency * 1.5,
            iops: (config.read_throughput * 100.0) as u32,
            supports_parallel_io: true,
            optimal_block_size: 4096,
        },
        available_space: config.capacity_gb * 1024 * 1024 * 1024,
        reliability_score: config.reliability,
        cost_profile: CostProfile {
            storage_cost_per_gb_month: config.cost_per_gb,
            request_cost_per_thousand: 0.0004,
            transfer_cost_per_gb: 0.09,
            is_free_tier: false,
        },
        metadata: HashMap::new(),
    }
}

// ==================== BASIC INSTANTIATION TESTS ====================

#[test]
fn test_storage_analyzer_new() {
    let _analyzer = StorageAnalyzer::new();
    // Should create successfully
}

#[test]
fn test_storage_analyzer_default() {
    let _analyzer = StorageAnalyzer::default();
}

#[test]
fn test_storage_analyzer_with_custom_thresholds() {
    let _analyzer = StorageAnalyzer::with_thresholds(15.0, 100.0);
}

#[test]
fn test_storage_analyzer_zero_thresholds() {
    let _analyzer = StorageAnalyzer::with_thresholds(0.0, 0.0);
}

#[test]
fn test_storage_analyzer_high_thresholds() {
    let _analyzer = StorageAnalyzer::with_thresholds(99.0, 10000.0);
}

// ==================== STORAGE ANALYSIS TESTS ====================

#[test]
fn test_analyze_empty_storage_list() {
    let analyzer = StorageAnalyzer::new();
    let storage_list: Vec<DetectedStorage> = vec![];

    let report = analyzer.analyze_storage_systems(&storage_list);

    assert_eq!(report.filesystem_total, 0);
    assert_eq!(report.filesystem_used, 0);
    assert_eq!(report.filesystem_usage_percent, 0.0);
}

#[test]
fn test_analyze_single_storage() {
    let analyzer = StorageAnalyzer::new();
    let storage = create_test_storage(
        "test",
        UnifiedStorageType::Local,
        500.0, // 500 MB/s throughput
        100.0, // 100 μs latency
        1000,  // 1TB capacity
        0.0,   // Free (local)
        0.95,  // 95% reliability
        vec![UnifiedStorageCapability::Encryption],
    );

    let report = analyzer.analyze_storage_systems(&[storage]);

    assert!(report.filesystem_total > 0);
    assert!(report.filesystem_usage_percent >= 0.0);
    assert!(report.filesystem_usage_percent <= 100.0);
}

#[test]
fn test_analyze_multiple_storage_systems() {
    let analyzer = StorageAnalyzer::new();
    let storage1 = create_test_storage(
        "fast_ssd",
        UnifiedStorageType::Local,
        1000.0,
        50.0,
        500,
        0.0,
        0.98,
        vec![UnifiedStorageCapability::Encryption],
    );
    let storage2 = create_test_storage(
        "slow_hdd",
        UnifiedStorageType::Local,
        100.0,
        5000.0,
        2000,
        0.0,
        0.90,
        vec![],
    );

    let report = analyzer.analyze_storage_systems(&[storage1, storage2]);

    // Should combine both systems
    assert!(report.filesystem_total > 0);
    assert!(!report.recommendations.is_empty()); // Should have recommendations
}

#[test]
fn test_analyze_storage_generates_low_throughput_recommendation() {
    let analyzer = StorageAnalyzer::new();
    let slow_storage = create_test_storage(
        "slow_storage",
        UnifiedStorageType::Local,
        10.0, // Very slow: 10 MB/s (< 50 MB/s threshold)
        100.0,
        100,
        0.0,
        0.95,
        vec![],
    );

    let report = analyzer.analyze_storage_systems(&[slow_storage]);

    // Should generate performance recommendation
    assert!(
        report
            .recommendations
            .iter()
            .any(|r| r.contains("low read throughput"))
    );
}

#[test]
fn test_analyze_storage_generates_high_latency_recommendation() {
    let analyzer = StorageAnalyzer::new();
    let high_latency_storage = create_test_storage(
        "remote_storage",
        UnifiedStorageType::Cloud,
        500.0,
        15000.0, // 15ms latency (> 10ms threshold)
        1000,
        0.05,
        0.95,
        vec![],
    );

    let report = analyzer.analyze_storage_systems(&[high_latency_storage]);

    // Should generate latency recommendation
    assert!(
        report
            .recommendations
            .iter()
            .any(|r| r.contains("high latency"))
    );
}

#[test]
fn test_analyze_storage_generates_encryption_recommendation() {
    let analyzer = StorageAnalyzer::new();
    let unencrypted_storage = create_test_storage(
        "unencrypted",
        UnifiedStorageType::Local,
        500.0,
        100.0,
        1000,
        0.0,
        0.95,
        vec![], // No encryption capability
    );

    let report = analyzer.analyze_storage_systems(&[unencrypted_storage]);

    // Should recommend encryption
    assert!(
        report
            .recommendations
            .iter()
            .any(|r| r.contains("encryption"))
    );
}

#[test]
fn test_analyze_storage_generates_reliability_recommendation() {
    let analyzer = StorageAnalyzer::new();
    let unreliable_storage = create_test_storage(
        "unreliable",
        UnifiedStorageType::Local,
        500.0,
        100.0,
        1000,
        0.0,
        0.5, // Low reliability (< 0.8 threshold)
        vec![],
    );

    let report = analyzer.analyze_storage_systems(&[unreliable_storage]);

    // Should recommend backup strategies
    assert!(
        report
            .recommendations
            .iter()
            .any(|r| r.contains("reliability"))
    );
}

#[test]
fn test_analyze_storage_generates_cost_recommendation() {
    let analyzer = StorageAnalyzer::new();
    let expensive_cloud_storage = create_test_storage(
        "expensive_cloud",
        UnifiedStorageType::Cloud,
        500.0,
        100.0,
        1000,
        0.10, // Expensive: $0.10/GB/month (> $0.05 threshold)
        0.95,
        vec![],
    );

    let report = analyzer.analyze_storage_systems(&[expensive_cloud_storage]);

    // Should recommend cheaper alternatives
    assert!(
        report
            .recommendations
            .iter()
            .any(|r| r.contains("high cost"))
    );
}

// ==================== USE CASE SCORING TESTS ====================

#[test]
fn test_recommend_storage_high_performance() {
    let analyzer = StorageAnalyzer::new();
    let fast_storage = create_test_storage(
        "fast",
        UnifiedStorageType::Local,
        2000.0, // Very fast
        50.0,   // Low latency
        500,
        0.0,
        0.95,
        vec![],
    );
    let slow_storage = create_test_storage(
        "slow",
        UnifiedStorageType::Local,
        50.0,
        10000.0,
        1000,
        0.0,
        0.95,
        vec![],
    );

    let storage_list = [fast_storage, slow_storage];
    let best =
        analyzer.recommend_storage_for_use_case(&storage_list, StorageUseCase::HighPerformance);

    assert!(best.is_some());
    assert_eq!(best.unwrap().display_name, "fast");
}

#[test]
fn test_recommend_storage_low_cost() {
    let analyzer = StorageAnalyzer::new();
    let local_storage = create_test_storage(
        "local",
        UnifiedStorageType::Local,
        500.0,
        100.0,
        1000,
        0.0, // Free
        0.95,
        vec![],
    );
    let expensive_cloud = create_test_storage(
        "cloud",
        UnifiedStorageType::Cloud,
        500.0,
        100.0,
        1000,
        0.10,
        0.95,
        vec![],
    );

    let storage_list = [local_storage, expensive_cloud];
    let best = analyzer.recommend_storage_for_use_case(&storage_list, StorageUseCase::LowCost);

    assert!(best.is_some());
    // Local storage should win for low cost
    assert_eq!(best.unwrap().display_name, "local");
}

#[test]
fn test_recommend_storage_high_capacity() {
    let analyzer = StorageAnalyzer::new();
    let small_storage = create_test_storage(
        "small",
        UnifiedStorageType::Local,
        500.0,
        100.0,
        100, // 100GB
        0.0,
        0.95,
        vec![],
    );
    let large_storage = create_test_storage(
        "large",
        UnifiedStorageType::Local,
        500.0,
        100.0,
        10000, // 10TB
        0.0,
        0.95,
        vec![],
    );

    let storage_list = [small_storage, large_storage];
    let best = analyzer.recommend_storage_for_use_case(&storage_list, StorageUseCase::HighCapacity);

    assert!(best.is_some());
    assert_eq!(best.unwrap().display_name, "large");
}

#[test]
fn test_recommend_storage_backup() {
    let analyzer = StorageAnalyzer::new();
    let unreliable = create_test_storage(
        "unreliable",
        UnifiedStorageType::Local,
        500.0,
        100.0,
        1000,
        0.0,
        0.70, // Low reliability
        vec![],
    );
    let reliable = create_test_storage(
        "reliable",
        UnifiedStorageType::Local,
        500.0,
        100.0,
        1000,
        0.0,
        0.99, // High reliability
        vec![
            UnifiedStorageCapability::Encryption,
            UnifiedStorageCapability::Compression,
        ],
    );

    let storage_list = [unreliable, reliable];
    let best = analyzer.recommend_storage_for_use_case(&storage_list, StorageUseCase::Backup);

    assert!(best.is_some());
    assert_eq!(best.unwrap().display_name, "reliable");
}

#[test]
fn test_recommend_storage_archive() {
    let analyzer = StorageAnalyzer::new();
    let small_expensive = create_test_storage(
        "small",
        UnifiedStorageType::Cloud,
        500.0,
        100.0,
        100,
        0.10,
        0.95,
        vec![],
    );
    let large_cheap = create_test_storage(
        "large_cheap",
        UnifiedStorageType::Cloud,
        500.0,
        100.0,
        5000,
        0.01, // Very cheap
        0.95,
        vec![],
    );

    let storage_list = [small_expensive, large_cheap];
    let best = analyzer.recommend_storage_for_use_case(&storage_list, StorageUseCase::Archive);

    assert!(best.is_some());
    assert_eq!(best.unwrap().display_name, "large_cheap");
}

#[test]
fn test_recommend_storage_empty_list() {
    let analyzer = StorageAnalyzer::new();
    let storage_list: Vec<DetectedStorage> = vec![];

    let best =
        analyzer.recommend_storage_for_use_case(&storage_list, StorageUseCase::HighPerformance);

    assert!(best.is_none());
}

// ==================== OPTIMIZATION SUGGESTIONS TESTS ====================

#[test]
fn test_generate_optimization_suggestions_empty() {
    let analyzer = StorageAnalyzer::new();
    let storage_list: Vec<DetectedStorage> = vec![];

    let suggestions = analyzer.generate_optimization_suggestions(&storage_list);

    // Should still work with empty list
    assert!(suggestions.is_empty());
}

#[test]
fn test_generate_optimization_suggestions_local_consolidation() {
    let analyzer = StorageAnalyzer::new();
    // Create 5 local storage systems (> 3 threshold)
    let mut storage_list = Vec::new();
    for i in 1..=5 {
        storage_list.push(create_test_storage(
            &format!("local{}", i),
            UnifiedStorageType::Local,
            500.0,
            100.0,
            1000,
            0.0,
            0.95,
            vec![],
        ));
    }

    let suggestions = analyzer.generate_optimization_suggestions(&storage_list);

    // Should suggest consolidation
    assert!(suggestions.iter().any(|s| s.contains("consolidating")));
}

#[test]
fn test_generate_optimization_suggestions_tiered_storage() {
    let analyzer = StorageAnalyzer::new();
    let local = create_test_storage(
        "local",
        UnifiedStorageType::Local,
        1000.0,
        50.0,
        500,
        0.0,
        0.95,
        vec![],
    );
    let cloud = create_test_storage(
        "cloud",
        UnifiedStorageType::Cloud,
        200.0,
        5000.0,
        10000,
        0.02,
        0.95,
        vec![],
    );

    let suggestions = analyzer.generate_optimization_suggestions(&[local, cloud]);

    // Should suggest tiered storage
    assert!(suggestions.iter().any(|s| s.contains("tiered storage")));
}

#[test]
fn test_generate_optimization_suggestions_encryption() {
    let analyzer = StorageAnalyzer::new();
    let unencrypted1 = create_test_storage(
        "unencrypted1",
        UnifiedStorageType::Local,
        500.0,
        100.0,
        1000,
        0.0,
        0.95,
        vec![], // No encryption
    );
    let unencrypted2 = create_test_storage(
        "unencrypted2",
        UnifiedStorageType::Local,
        500.0,
        100.0,
        1000,
        0.0,
        0.95,
        vec![], // No encryption
    );

    let suggestions = analyzer.generate_optimization_suggestions(&[unencrypted1, unencrypted2]);

    // Should suggest enabling encryption
    assert!(suggestions.iter().any(|s| s.contains("encryption")));
}

// ==================== EFFICIENCY SCORING TESTS ====================

#[test]
fn test_calculate_efficiency_score_high_performance() {
    let analyzer = StorageAnalyzer::new();
    let fast_storage = create_test_storage(
        "fast",
        UnifiedStorageType::Local,
        2000.0, // High throughput
        50.0,   // Low latency
        1000,
        0.0,
        0.95,
        vec![UnifiedStorageCapability::Compression],
    );

    let score = analyzer.calculate_efficiency_score(&fast_storage);

    assert!(score > 0.0);
    assert!(score <= 1.0); // Capped at 1.0
}

#[test]
fn test_calculate_efficiency_score_cloud_storage() {
    let analyzer = StorageAnalyzer::new();
    let cloud = create_test_storage(
        "cloud",
        UnifiedStorageType::Cloud,
        500.0,
        1000.0,
        1000,
        0.02, // Low cost
        0.95,
        vec![],
    );

    let score = analyzer.calculate_efficiency_score(&cloud);

    assert!(score > 0.0);
    assert!(score <= 1.0);
}

#[test]
fn test_calculate_efficiency_score_with_compression() {
    let analyzer = StorageAnalyzer::new();
    let with_compression = create_test_storage(
        "compressed",
        UnifiedStorageType::Local,
        500.0,
        100.0,
        1000,
        0.0,
        0.95,
        vec![UnifiedStorageCapability::Compression],
    );
    let without_compression = create_test_storage(
        "uncompressed",
        UnifiedStorageType::Local,
        500.0,
        100.0,
        1000,
        0.0,
        0.95,
        vec![],
    );

    let score_with = analyzer.calculate_efficiency_score(&with_compression);
    let score_without = analyzer.calculate_efficiency_score(&without_compression);

    // Both scores should be valid (>= 0.0 and <= 1.0)
    assert!((0.0..=1.0).contains(&score_with));
    assert!((0.0..=1.0).contains(&score_without));
    // Compression should provide at least some value
    assert!(score_with >= score_without);
}

// ==================== REPORT SUMMARY TESTS ====================

#[test]
fn test_analysis_report_generate_summary() {
    let analyzer = StorageAnalyzer::new();
    let storage = create_test_storage(
        "test",
        UnifiedStorageType::Local,
        500.0,
        100.0,
        1000,
        0.0,
        0.95,
        vec![],
    );

    let report = analyzer.analyze_storage_systems(&[storage]);
    let summary = report.generate_summary();

    assert!(!summary.is_empty());
    assert!(summary.contains("Storage Analysis Summary"));
    assert!(summary.contains("Total Filesystem Space"));
}

#[test]
fn test_analysis_report_summary_contains_recommendations() {
    let analyzer = StorageAnalyzer::new();
    let slow_storage = create_test_storage(
        "slow",
        UnifiedStorageType::Local,
        10.0, // Very slow
        100.0,
        1000,
        0.0,
        0.95,
        vec![],
    );

    let report = analyzer.analyze_storage_systems(&[slow_storage]);
    let summary = report.generate_summary();

    assert!(summary.contains("Recommendations"));
    assert!(summary.contains("Detailed Recommendations"));
}

// ==================== EDGE CASES AND BOUNDARY CONDITIONS ====================

#[test]
fn test_analyzer_with_negative_thresholds() {
    let analyzer = StorageAnalyzer::with_thresholds(-10.0, -100.0);
    let storage = create_test_storage(
        "test",
        UnifiedStorageType::Local,
        500.0,
        100.0,
        1000,
        0.0,
        0.95,
        vec![],
    );

    let report = analyzer.analyze_storage_systems(&[storage]);
    // Should not crash
    assert!(report.filesystem_total > 0);
}

#[test]
fn test_efficiency_score_zero_latency() {
    let analyzer = StorageAnalyzer::new();
    let storage = create_test_storage(
        "zero_latency",
        UnifiedStorageType::Local,
        1000.0,
        0.0, // Zero latency (unrealistic but shouldn't crash)
        1000,
        0.0,
        0.95,
        vec![],
    );

    let score = analyzer.calculate_efficiency_score(&storage);
    assert!(score >= 0.0);
}

#[test]
fn test_scoring_identical_storage() {
    let analyzer = StorageAnalyzer::new();
    let storage1 = create_test_storage(
        "storage1",
        UnifiedStorageType::Local,
        500.0,
        100.0,
        1000,
        0.0,
        0.95,
        vec![],
    );
    let storage2 = create_test_storage(
        "storage2",
        UnifiedStorageType::Local,
        500.0,
        100.0,
        1000,
        0.0,
        0.95,
        vec![],
    );

    let storage_list = [storage1, storage2];
    let best =
        analyzer.recommend_storage_for_use_case(&storage_list, StorageUseCase::HighPerformance);

    // Should still return one (first one with highest score)
    assert!(best.is_some());
}

#[test]
fn test_all_storage_types_supported() {
    let analyzer = StorageAnalyzer::new();
    let storage_types = vec![
        UnifiedStorageType::Local,
        UnifiedStorageType::Zfs,
        UnifiedStorageType::Cloud,
        UnifiedStorageType::Network,
    ];

    for storage_type in storage_types {
        let storage =
            create_test_storage("test", storage_type, 500.0, 100.0, 1000, 0.02, 0.95, vec![]);

        let report = analyzer.analyze_storage_systems(&[storage]);
        // Should handle all storage types
        assert!(report.filesystem_total > 0);
    }
}
