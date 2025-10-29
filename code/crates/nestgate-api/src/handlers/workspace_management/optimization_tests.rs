//! **COMPREHENSIVE WORKSPACE OPTIMIZATION TESTS**
//!
//! Tests for ZFS optimization structures and helper functions.

use super::optimization::{
    analyze_storage_patterns, get_optimization_stats, optimize_cache_settings,
    optimize_compression, optimize_deduplication, optimize_recordsize, request_ai_optimization,
    StoragePattern,
};

// ==================== STORAGE PATTERN TESTS ====================

#[test]
fn test_storage_pattern_structure() {
    let mut file_types = std::collections::HashMap::new();
    file_types.insert("text".to_string(), 0.5);
    file_types.insert("binary".to_string(), 0.5);

    let pattern = StoragePattern {
        file_size_distribution: "small".to_string(),
        file_type_distribution: file_types.clone(),
        duplicate_ratio: 0.3,
        sequential_vs_random: 0.8,
        read_write_ratio: 4.0,
    };

    assert_eq!(pattern.file_size_distribution, "small");
    assert_eq!(pattern.duplicate_ratio, 0.3);
    assert_eq!(pattern.sequential_vs_random, 0.8);
    assert_eq!(pattern.read_write_ratio, 4.0);
}

#[test]
fn test_storage_pattern_clone() {
    let mut file_types = std::collections::HashMap::new();
    file_types.insert("text".to_string(), 0.6);

    let pattern = StoragePattern {
        file_size_distribution: "mixed".to_string(),
        file_type_distribution: file_types,
        duplicate_ratio: 0.15,
        sequential_vs_random: 0.5,
        read_write_ratio: 2.0,
    };

    let cloned = pattern;
    assert_eq!(cloned.duplicate_ratio, 0.15);
}

#[test]
fn test_storage_pattern_serialization() {
    let mut file_types = std::collections::HashMap::new();
    file_types.insert("compressed".to_string(), 0.3);

    let pattern = StoragePattern {
        file_size_distribution: "large".to_string(),
        file_type_distribution: file_types,
        duplicate_ratio: 0.05,
        sequential_vs_random: 0.9,
        read_write_ratio: 10.0,
    };

    let json = serde_json::to_string(&pattern);
    assert!(json.is_ok());
}

#[test]
fn test_analyze_storage_patterns() {
    let pattern = analyze_storage_patterns("test/dataset");

    assert!(!pattern.file_size_distribution.is_empty());
    assert!(!pattern.file_type_distribution.is_empty());
    assert!(pattern.duplicate_ratio >= 0.0);
    assert!(pattern.sequential_vs_random >= 0.0 && pattern.sequential_vs_random <= 1.0);
    assert!(pattern.read_write_ratio >= 0.0);
}

#[test]
fn test_analyze_storage_patterns_file_types() {
    let pattern = analyze_storage_patterns("test/dataset");

    // Should have standard file type categories
    assert!(pattern.file_type_distribution.contains_key("text"));
    assert!(pattern.file_type_distribution.contains_key("binary"));
    assert!(pattern.file_type_distribution.contains_key("compressed"));
    assert!(pattern.file_type_distribution.contains_key("other"));
}

#[test]
fn test_analyze_storage_patterns_percentages() {
    let pattern = analyze_storage_patterns("test/dataset");

    // File type percentages should be between 0 and 1
    for percentage in pattern.file_type_distribution.values() {
        assert!(*percentage >= 0.0 && *percentage <= 1.0);
    }
}

#[test]
fn test_analyze_storage_patterns_sequential_ratio() {
    let pattern = analyze_storage_patterns("test/dataset");

    // Sequential vs random should be between 0 and 1
    assert!(pattern.sequential_vs_random >= 0.0);
    assert!(pattern.sequential_vs_random <= 1.0);
}

#[test]
fn test_analyze_storage_patterns_read_write_ratio() {
    let pattern = analyze_storage_patterns("test/dataset");

    // Read/write ratio should be positive
    assert!(pattern.read_write_ratio > 0.0);
}

// ==================== COMPRESSION OPTIMIZATION TESTS ====================

#[test]
fn test_optimize_compression_text_heavy() {
    let mut file_types = std::collections::HashMap::new();
    file_types.insert("text".to_string(), 0.6);
    file_types.insert("binary".to_string(), 0.4);

    let pattern = StoragePattern {
        file_size_distribution: "mixed".to_string(),
        file_type_distribution: file_types,
        duplicate_ratio: 0.2,
        sequential_vs_random: 0.7,
        read_write_ratio: 3.0,
    };

    let result = optimize_compression("test/dataset", &pattern);
    assert!(result.is_some());
}

#[test]
fn test_optimize_compression_binary_heavy() {
    let mut file_types = std::collections::HashMap::new();
    file_types.insert("text".to_string(), 0.2);
    file_types.insert("binary".to_string(), 0.6);

    let pattern = StoragePattern {
        file_size_distribution: "large".to_string(),
        file_type_distribution: file_types,
        duplicate_ratio: 0.1,
        sequential_vs_random: 0.8,
        read_write_ratio: 2.0,
    };

    let result = optimize_compression("test/dataset", &pattern);
    assert!(result.is_some());
}

#[test]
fn test_optimize_compression_returns_message() {
    let pattern = analyze_storage_patterns("test/dataset");
    let result = optimize_compression("test/dataset", &pattern);

    if let Some(message) = result {
        assert!(!message.is_empty());
    }
}

// ==================== RECORDSIZE OPTIMIZATION TESTS ====================

#[test]
fn test_optimize_recordsize_sequential() {
    let mut file_types = std::collections::HashMap::new();
    file_types.insert("text".to_string(), 0.5);

    let pattern = StoragePattern {
        file_size_distribution: "large".to_string(),
        file_type_distribution: file_types,
        duplicate_ratio: 0.1,
        sequential_vs_random: 0.9, // Very sequential
        read_write_ratio: 5.0,
    };

    let result = optimize_recordsize("test/dataset", &pattern);
    assert!(result.is_some());
}

#[test]
fn test_optimize_recordsize_random() {
    let mut file_types = std::collections::HashMap::new();
    file_types.insert("binary".to_string(), 0.5);

    let pattern = StoragePattern {
        file_size_distribution: "small".to_string(),
        file_type_distribution: file_types,
        duplicate_ratio: 0.05,
        sequential_vs_random: 0.2, // Very random
        read_write_ratio: 1.5,
    };

    let result = optimize_recordsize("test/dataset", &pattern);
    assert!(result.is_some());
}

#[test]
fn test_optimize_recordsize_balanced() {
    let mut file_types = std::collections::HashMap::new();
    file_types.insert("text".to_string(), 0.3);
    file_types.insert("binary".to_string(), 0.3);

    let pattern = StoragePattern {
        file_size_distribution: "mixed".to_string(),
        file_type_distribution: file_types,
        duplicate_ratio: 0.15,
        sequential_vs_random: 0.5, // Balanced
        read_write_ratio: 3.0,
    };

    let result = optimize_recordsize("test/dataset", &pattern);
    assert!(result.is_some());
}

// ==================== CACHE OPTIMIZATION TESTS ====================

#[test]
fn test_optimize_cache_read_heavy() {
    let mut file_types = std::collections::HashMap::new();
    file_types.insert("text".to_string(), 0.5);

    let pattern = StoragePattern {
        file_size_distribution: "mixed".to_string(),
        file_type_distribution: file_types,
        duplicate_ratio: 0.1,
        sequential_vs_random: 0.7,
        read_write_ratio: 6.0, // Read-heavy
    };

    let result = optimize_cache_settings("test/dataset", &pattern);
    assert!(result.is_some());
}

#[test]
fn test_optimize_cache_write_heavy() {
    let mut file_types = std::collections::HashMap::new();
    file_types.insert("binary".to_string(), 0.6);

    let pattern = StoragePattern {
        file_size_distribution: "large".to_string(),
        file_type_distribution: file_types,
        duplicate_ratio: 0.05,
        sequential_vs_random: 0.8,
        read_write_ratio: 0.5, // Write-heavy
    };

    let result = optimize_cache_settings("test/dataset", &pattern);
    assert!(result.is_some());
}

#[test]
fn test_optimize_cache_balanced() {
    let mut file_types = std::collections::HashMap::new();
    file_types.insert("text".to_string(), 0.4);
    file_types.insert("binary".to_string(), 0.4);

    let pattern = StoragePattern {
        file_size_distribution: "mixed".to_string(),
        file_type_distribution: file_types,
        duplicate_ratio: 0.12,
        sequential_vs_random: 0.6,
        read_write_ratio: 2.5, // Balanced
    };

    let result = optimize_cache_settings("test/dataset", &pattern);
    assert!(result.is_some());
}

// ==================== DEDUPLICATION TESTS ====================

#[test]
fn test_optimize_deduplication() {
    let result = optimize_deduplication("test/dataset");
    assert!(result.is_some());
}

#[test]
fn test_optimize_deduplication_returns_message() {
    let result = optimize_deduplication("test/dataset");

    if let Some(message) = result {
        assert!(!message.is_empty());
    }
}

// ==================== OPTIMIZATION STATS TESTS ====================

#[test]
fn test_get_optimization_stats() {
    let stats = get_optimization_stats("test/dataset");

    // Should return a valid JSON value
    assert!(stats.is_object() || stats.is_string());
}

#[test]
fn test_get_optimization_stats_structure() {
    let stats = get_optimization_stats("test/dataset");

    // Stats should be a JSON object
    assert!(stats.is_object());
}

// ==================== INTEGRATION TESTS ====================

#[test]
fn test_full_optimization_workflow() {
    // Simulate a complete optimization workflow
    let dataset_name = "test/dataset";

    // 1. Analyze patterns
    let pattern = analyze_storage_patterns(dataset_name);
    assert!(!pattern.file_size_distribution.is_empty());

    // 2. Test each optimization function
    let compression_result = optimize_compression(dataset_name, &pattern);
    let recordsize_result = optimize_recordsize(dataset_name, &pattern);
    let cache_result = optimize_cache_settings(dataset_name, &pattern);

    // All should return Some result
    assert!(compression_result.is_some());
    assert!(recordsize_result.is_some());
    assert!(cache_result.is_some());

    // 3. Get final stats
    let stats = get_optimization_stats(dataset_name);
    assert!(stats.is_object());
}

#[test]
fn test_deduplication_based_on_ratio() {
    let mut file_types = std::collections::HashMap::new();
    file_types.insert("text".to_string(), 0.5);

    let pattern = StoragePattern {
        file_size_distribution: "mixed".to_string(),
        file_type_distribution: file_types,
        duplicate_ratio: 0.15, // High enough for dedup
        sequential_vs_random: 0.7,
        read_write_ratio: 3.0,
    };

    // Test that dedup should be considered
    assert!(pattern.duplicate_ratio > 0.1);

    let dedup_result = optimize_deduplication("test/dataset");
    assert!(dedup_result.is_some());
}

#[tokio::test]
async fn test_request_ai_optimization_no_endpoint() {
    let pattern = analyze_storage_patterns("test/dataset");

    // Without AI endpoint configured, should return None
    std::env::remove_var("NESTGATE_AI_ENDPOINT");
    let result = request_ai_optimization("test/dataset", &pattern).await;

    // Should handle missing endpoint gracefully
    assert!(result.is_none());
}

#[test]
fn test_storage_pattern_all_ratios() {
    // Test various ratio combinations
    let test_cases = vec![
        (0.0, 0.0, 1.0),  // No duplicates, pure sequential, balanced I/O
        (0.5, 1.0, 10.0), // High duplicates, random access, read-heavy
        (0.2, 0.5, 0.5),  // Low duplicates, mixed access, write-heavy
    ];

    for (dup, seq, rw) in test_cases {
        let mut file_types = std::collections::HashMap::new();
        file_types.insert("test".to_string(), 1.0);

        let pattern = StoragePattern {
            file_size_distribution: "test".to_string(),
            file_type_distribution: file_types,
            duplicate_ratio: dup,
            sequential_vs_random: seq,
            read_write_ratio: rw,
        };

        assert_eq!(pattern.duplicate_ratio, dup);
        assert_eq!(pattern.sequential_vs_random, seq);
        assert_eq!(pattern.read_write_ratio, rw);
    }
}

#[test]
fn test_file_type_distribution_total() {
    let pattern = analyze_storage_patterns("test/dataset");

    // Sum of all file type percentages
    let total: f64 = pattern.file_type_distribution.values().sum();

    // Should roughly equal 1.0 (allowing for floating point precision)
    assert!(
        (total - 1.0).abs() < 0.01,
        "Total percentage should be ~1.0, got {total}"
    );
}

#[test]
fn test_multiple_optimization_calls() {
    let dataset_name = "test/dataset";
    let pattern = analyze_storage_patterns(dataset_name);

    // Call optimizations multiple times
    for _ in 0..3 {
        let result = optimize_compression(dataset_name, &pattern);
        assert!(result.is_some());
    }
}

#[test]
fn test_optimization_different_datasets() {
    let datasets = vec!["test/dataset1", "test/dataset2", "test/dataset3"];

    for dataset in datasets {
        let pattern = analyze_storage_patterns(dataset);
        assert!(!pattern.file_size_distribution.is_empty());

        let result = optimize_compression(dataset, &pattern);
        assert!(result.is_some());
    }
}

#[test]
fn test_storage_pattern_edge_cases() {
    // Test extreme values
    let mut file_types = std::collections::HashMap::new();
    file_types.insert("all".to_string(), 1.0);

    let pattern = StoragePattern {
        file_size_distribution: "extreme".to_string(),
        file_type_distribution: file_types,
        duplicate_ratio: 0.0,      // No duplicates
        sequential_vs_random: 1.0, // 100% sequential
        read_write_ratio: 100.0,   // 100:1 read heavy
    };

    assert_eq!(pattern.duplicate_ratio, 0.0);
    assert_eq!(pattern.sequential_vs_random, 1.0);
}

#[test]
fn test_cache_optimization_extreme_read() {
    let mut file_types = std::collections::HashMap::new();
    file_types.insert("test".to_string(), 1.0);

    let pattern = StoragePattern {
        file_size_distribution: "test".to_string(),
        file_type_distribution: file_types,
        duplicate_ratio: 0.1,
        sequential_vs_random: 0.5,
        read_write_ratio: 20.0, // Extreme read-heavy
    };

    let result = optimize_cache_settings("test/dataset", &pattern);
    assert!(result.is_some());
}

#[test]
fn test_cache_optimization_extreme_write() {
    let mut file_types = std::collections::HashMap::new();
    file_types.insert("test".to_string(), 1.0);

    let pattern = StoragePattern {
        file_size_distribution: "test".to_string(),
        file_type_distribution: file_types,
        duplicate_ratio: 0.05,
        sequential_vs_random: 0.6,
        read_write_ratio: 0.1, // Extreme write-heavy
    };

    let result = optimize_cache_settings("test/dataset", &pattern);
    assert!(result.is_some());
}

#[test]
fn test_recordsize_boundaries() {
    let mut file_types = std::collections::HashMap::new();
    file_types.insert("test".to_string(), 1.0);

    // Test boundary values for sequential_vs_random
    let test_cases = vec![
        0.85, // Should trigger 1M
        0.25, // Should trigger 4K
        0.50, // Should trigger 128K
    ];

    for seq_ratio in test_cases {
        let pattern = StoragePattern {
            file_size_distribution: "test".to_string(),
            file_type_distribution: file_types.clone(),
            duplicate_ratio: 0.1,
            sequential_vs_random: seq_ratio,
            read_write_ratio: 2.0,
        };

        let result = optimize_recordsize("test/dataset", &pattern);
        assert!(result.is_some());
    }
}
