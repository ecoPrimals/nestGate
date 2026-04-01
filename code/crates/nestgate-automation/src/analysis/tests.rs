// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

use std::collections::HashMap;
use std::time::SystemTime;

use nestgate_core::unified_enums::StorageTier;

use crate::types::prediction::{
    AccessPattern, AccessType, DataPattern, FileAnalysis, SizeCategory,
};

use super::{
    DatasetAnalysis, DatasetAnalyzer, DatasetSummary, FileAnalyzer, FileCharacteristics,
    PatternAnalyzer,
};

#[test]
fn test_file_characteristics_creation() {
    let characteristics = FileCharacteristics {
        size_category: SizeCategory::Medium,
        access_frequency: 50,
        is_frequently_accessed: true,
        is_sequential_access: false,
        data_pattern: DataPattern::Random,
    };
    assert_eq!(characteristics.access_frequency, 50);
    assert!(characteristics.is_frequently_accessed);
}

#[test]
fn test_dataset_analysis_creation() {
    let analysis = DatasetAnalysis {
        path: "/tank/data".to_string(),
        total_files: 1000,
        total_size_bytes: 1024 * 1024 * 1024,
        file_types: HashMap::new(),
        characteristics: FileCharacteristics {
            size_category: SizeCategory::Large,
            access_frequency: 100,
            is_frequently_accessed: true,
            is_sequential_access: true,
            data_pattern: DataPattern::Sequential,
        },
    };
    assert_eq!(analysis.path, "/tank/data");
    assert_eq!(analysis.total_files, 1000);
}

#[test]
fn test_file_analyzer_new() {
    let analyzer = FileAnalyzer::new();
    drop(analyzer);
}

#[test]
fn test_dataset_analyzer_new() {
    let analyzer = DatasetAnalyzer::new();
    drop(analyzer);
}

#[test]
fn test_file_characteristics_serialization() {
    let characteristics = FileCharacteristics {
        size_category: SizeCategory::Small,
        access_frequency: 10,
        is_frequently_accessed: false,
        is_sequential_access: true,
        data_pattern: DataPattern::Sequential,
    };
    let serialized = serde_json::to_string(&characteristics)
        .expect("Test: characteristics serialization should succeed");
    assert!(serialized.contains("access_frequency"));
}

#[test]
fn test_dataset_analysis_with_file_types() {
    let mut file_types = HashMap::new();
    file_types.insert("txt".to_string(), 500);
    file_types.insert("pdf".to_string(), 300);

    let analysis = DatasetAnalysis {
        path: "/data".to_string(),
        total_files: 800,
        total_size_bytes: 500 * 1024 * 1024,
        file_types,
        characteristics: FileCharacteristics {
            size_category: SizeCategory::Medium,
            access_frequency: 25,
            is_frequently_accessed: false,
            is_sequential_access: false,
            data_pattern: DataPattern::Mixed,
        },
    };
    assert_eq!(analysis.file_types.len(), 2);
    assert_eq!(
        *analysis
            .file_types
            .get("txt")
            .expect("Test: file_types should contain 'txt'"),
        500
    );
}

#[test]
fn test_file_characteristics_various_patterns() {
    let sequential = FileCharacteristics {
        size_category: SizeCategory::Large,
        access_frequency: 200,
        is_frequently_accessed: true,
        is_sequential_access: true,
        data_pattern: DataPattern::Sequential,
    };
    assert!(sequential.is_sequential_access);

    let random = FileCharacteristics {
        size_category: SizeCategory::Small,
        access_frequency: 5,
        is_frequently_accessed: false,
        is_sequential_access: false,
        data_pattern: DataPattern::Random,
    };
    assert!(!random.is_sequential_access);
}

#[test]
fn test_dataset_analysis_clone() {
    let analysis1 = DatasetAnalysis {
        path: "/test".to_string(),
        total_files: 100,
        total_size_bytes: 1024,
        file_types: HashMap::new(),
        characteristics: FileCharacteristics {
            size_category: SizeCategory::Small,
            access_frequency: 10,
            is_frequently_accessed: false,
            is_sequential_access: false,
            data_pattern: DataPattern::Mixed,
        },
    };
    let analysis2 = analysis1.clone();
    assert_eq!(analysis1.path, analysis2.path);
}

#[test]
fn test_file_characteristics_clone() {
    let char1 = FileCharacteristics {
        size_category: SizeCategory::Medium,
        access_frequency: 50,
        is_frequently_accessed: true,
        is_sequential_access: true,
        data_pattern: DataPattern::Sequential,
    };
    let char2 = char1.clone();
    assert_eq!(char1.access_frequency, char2.access_frequency);
}

#[test]
fn test_dataset_analysis_empty_file_types() {
    let analysis = DatasetAnalysis {
        path: "/empty".to_string(),
        total_files: 0,
        total_size_bytes: 0,
        file_types: HashMap::new(),
        characteristics: FileCharacteristics {
            size_category: SizeCategory::Small,
            access_frequency: 0,
            is_frequently_accessed: false,
            is_sequential_access: false,
            data_pattern: DataPattern::Mixed,
        },
    };
    assert!(analysis.file_types.is_empty());
    assert_eq!(analysis.total_files, 0);
}

#[test]
fn test_file_characteristics_high_frequency() {
    let characteristics = FileCharacteristics {
        size_category: SizeCategory::Large,
        access_frequency: 1000,
        is_frequently_accessed: true,
        is_sequential_access: true,
        data_pattern: DataPattern::Sequential,
    };
    assert_eq!(characteristics.access_frequency, 1000);
    assert!(characteristics.is_frequently_accessed);
}

#[test]
fn test_dataset_analysis_large_dataset() {
    let analysis = DatasetAnalysis {
        path: "/large".to_string(),
        total_files: 1_000_000,
        total_size_bytes: 1024 * 1024 * 1024 * 100,
        file_types: HashMap::new(),
        characteristics: FileCharacteristics {
            size_category: SizeCategory::Large,
            access_frequency: 500,
            is_frequently_accessed: true,
            is_sequential_access: true,
            data_pattern: DataPattern::Sequential,
        },
    };
    assert!(analysis.total_files >= 1_000_000);
    assert!(analysis.total_size_bytes > 1024 * 1024 * 1024);
}

#[test]
fn test_pattern_analyzer_new() {
    let analyzer = PatternAnalyzer::new();
    drop(analyzer);
}

#[test]
fn test_pattern_analyzer_default() {
    let analyzer = PatternAnalyzer::default();
    drop(analyzer);
}

#[tokio::test]
async fn test_pattern_analyzer_record_and_get() {
    let analyzer = PatternAnalyzer::new();
    analyzer
        .record_access("/path/file.txt", AccessType::Read)
        .await;
    let patterns = analyzer.get_patterns("/path/file.txt").await;
    assert_eq!(patterns.len(), 1);
    assert!(matches!(patterns[0].access_type, AccessType::Read));
}

#[tokio::test]
async fn test_pattern_analyzer_get_nonexistent() {
    let analyzer = PatternAnalyzer::new();
    let patterns = analyzer.get_patterns("/nonexistent").await;
    assert!(patterns.is_empty());
}

#[test]
fn test_pattern_analyzer_recommend_tier_log() {
    let analyzer = PatternAnalyzer::new();
    let tier = analyzer.recommend_tier("/var/log/app.log");
    assert_eq!(tier, StorageTier::Cold);
}

#[test]
fn test_pattern_analyzer_recommend_tier_doc() {
    let analyzer = PatternAnalyzer::new();
    let tier = analyzer.recommend_tier("/docs/file.pdf");
    assert_eq!(tier, StorageTier::Warm);
}

#[test]
fn test_pattern_analyzer_recommend_tier_video() {
    let analyzer = PatternAnalyzer::new();
    let tier = analyzer.recommend_tier("/media/movie.mp4");
    assert_eq!(tier, StorageTier::Hot);
}

#[test]
fn test_dataset_analyzer_default() {
    let analyzer = DatasetAnalyzer::default();
    drop(analyzer);
}

#[test]
fn test_dataset_analyzer_predict_optimal_tier_archive() {
    let analyzer = DatasetAnalyzer::new();
    let tier = analyzer.predict_optimal_tier("/data/archive").unwrap();
    assert_eq!(tier, StorageTier::Cold);
}

#[test]
fn test_dataset_analyzer_predict_optimal_tier_active() {
    let analyzer = DatasetAnalyzer::new();
    let tier = analyzer.predict_optimal_tier("/data/active").unwrap();
    assert_eq!(tier, StorageTier::Hot);
}

#[test]
fn test_dataset_analyzer_predict_optimal_tier_default() {
    let analyzer = DatasetAnalyzer::new();
    let tier = analyzer.predict_optimal_tier("/data/misc").unwrap();
    assert_eq!(tier, StorageTier::Warm);
}

#[tokio::test]
async fn test_dataset_analyzer_analyze_nonexistent() {
    let analyzer = DatasetAnalyzer::new();
    let result = analyzer.analyze_dataset("/nonexistent/path/xyz").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_dataset_analyzer_aggregate_patterns_empty() {
    let analyzer = DatasetAnalyzer::new();
    let result = analyzer.aggregate_patterns(&[], &AccessPattern::default());
    assert!(result.is_ok());
    let pattern = result.unwrap();
    assert_eq!(pattern.total_accesses, 0);
}

#[test]
fn test_dataset_summary_creation() {
    let summary = DatasetSummary {
        dataset_name: "pool/data".to_string(),
        total_files: 100,
        total_size_bytes: 1024 * 1024,
        average_file_size: 10240,
        file_types: HashMap::new(),
        access_pattern: AccessPattern::default(),
        compressible_files: 50,
        dedupable_files: 10,
    };
    assert_eq!(summary.dataset_name, "pool/data");
    assert_eq!(summary.total_files, 100);
    assert_eq!(summary.compressible_files, 50);
}

#[test]
fn test_dataset_summary_serialization() {
    let summary = DatasetSummary {
        dataset_name: "pool/data".to_string(),
        total_files: 50,
        total_size_bytes: 1024,
        average_file_size: 20,
        file_types: HashMap::new(),
        access_pattern: AccessPattern::default(),
        compressible_files: 5,
        dedupable_files: 2,
    };
    let serialized = serde_json::to_string(&summary).unwrap();
    assert!(serialized.contains("dataset_name"));
}

#[tokio::test]
async fn test_dataset_analyzer_aggregate_patterns_non_empty() {
    let analyzer = DatasetAnalyzer::new();
    let fa = FileAnalysis {
        file_path: "/tmp/a".into(),
        size_bytes: 500,
        created_at: SystemTime::now(),
        modified_at: SystemTime::now(),
        accessed_at: SystemTime::now(),
        file_type: "file".into(),
    };
    let out = analyzer
        .aggregate_patterns(&[&fa], &AccessPattern::default())
        .expect("aggregate");
    assert_eq!(out.total_accesses, 10);
    assert_eq!(out.accesses_last_24h, 10);
}

#[test]
fn pattern_analyzer_recommend_tier_by_extension() {
    let pa = PatternAnalyzer::new();
    assert_eq!(pa.recommend_tier("/var/log/app.log"), StorageTier::Cold);
    assert_eq!(pa.recommend_tier("doc/readme.pdf"), StorageTier::Warm);
    assert_eq!(pa.recommend_tier("video.mp4"), StorageTier::Hot);
}

#[test]
fn dataset_analyzer_predict_optimal_tier_paths() {
    let da = DatasetAnalyzer::new();
    assert_eq!(
        da.predict_optimal_tier("/archive/backup").unwrap(),
        StorageTier::Cold
    );
    assert_eq!(
        da.predict_optimal_tier("/active/current").unwrap(),
        StorageTier::Hot
    );
    assert_eq!(
        da.predict_optimal_tier("/data/misc").unwrap(),
        StorageTier::Warm
    );
}

#[tokio::test]
async fn file_analyzer_analyze_file_characteristics_compressible() {
    let dir = tempfile::tempdir().expect("tmp");
    let p = dir.path().join("note.txt");
    tokio::fs::write(&p, b"hello").await.expect("write");
    let fa = FileAnalyzer::new();
    let ch = fa
        .analyze_file_characteristics(p.to_str().unwrap())
        .await
        .expect("chars");
    assert!(matches!(
        ch.size_category,
        crate::types::prediction::SizeCategory::Small
    ));
}
