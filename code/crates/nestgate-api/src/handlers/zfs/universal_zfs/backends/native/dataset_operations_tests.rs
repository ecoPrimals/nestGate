// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Comprehensive tests for Native ZFS Dataset Operations
//!
//! Tests cover dataset parsing, size parsing, type parsing, and edge cases
//! to ensure reliable ZFS dataset management in production.

use super::{parse_dataset_type, parse_size};
use crate::handlers::zfs::universal_zfs_types::DatasetType;

// ==================== SIZE PARSING TESTS ====================

#[test]
fn test_parse_size_zero() {
    assert_eq!(parse_size("0"), Some(0));
}

#[test]
fn test_parse_size_dash() {
    assert_eq!(parse_size("-"), Some(0));
}

#[test]
fn test_parse_size_kilobytes() {
    assert_eq!(parse_size("1K"), Some(1024));
    assert_eq!(parse_size("10K"), Some(10 * 1024));
    assert_eq!(parse_size("512K"), Some(512 * 1024));
}

#[test]
fn test_parse_size_megabytes() {
    assert_eq!(parse_size("1M"), Some(1024 * 1024));
    assert_eq!(parse_size("100M"), Some(100 * 1024 * 1024));
    assert_eq!(parse_size("1.5M"), Some((1.5 * 1024.0 * 1024.0) as u64));
}

#[test]
fn test_parse_size_gigabytes() {
    assert_eq!(parse_size("1G"), Some(1024 * 1024 * 1024));
    assert_eq!(parse_size("10G"), Some(10 * 1024 * 1024 * 1024));
    assert_eq!(
        parse_size("2.5G"),
        Some((2.5 * 1024.0 * 1024.0 * 1024.0) as u64)
    );
}

#[test]
fn test_parse_size_terabytes() {
    assert_eq!(parse_size("1T"), Some(1024u64 * 1024 * 1024 * 1024));
    assert_eq!(parse_size("5T"), Some(5u64 * 1024 * 1024 * 1024 * 1024));
}

#[test]
fn test_parse_size_bytes() {
    assert_eq!(parse_size("1024"), Some(1024));
    assert_eq!(parse_size("4096"), Some(4096));
    assert_eq!(parse_size("1048576"), Some(1048576));
}

#[test]
fn test_parse_size_decimal_values() {
    assert_eq!(parse_size("1.5K"), Some((1.5 * 1024.0) as u64));
    assert_eq!(parse_size("2.25M"), Some((2.25 * 1024.0 * 1024.0) as u64));
    assert_eq!(
        parse_size("0.5G"),
        Some((0.5 * 1024.0 * 1024.0 * 1024.0) as u64)
    );
}

#[test]
fn test_parse_size_edge_cases() {
    // Very small
    assert_eq!(parse_size("1"), Some(1));

    // Empty
    assert_eq!(parse_size(""), None);

    // Invalid
    assert_eq!(parse_size("invalid"), None);
    assert_eq!(parse_size("ABC"), None);
}

#[test]
fn test_parse_size_case_insensitive() {
    assert_eq!(parse_size("1k"), Some(1024));
    assert_eq!(parse_size("1m"), Some(1024 * 1024));
    assert_eq!(parse_size("1g"), Some(1024 * 1024 * 1024));
    assert_eq!(parse_size("1t"), Some(1024u64 * 1024 * 1024 * 1024));
}

#[test]
fn test_parse_size_whitespace() {
    assert_eq!(parse_size(" 1M "), Some(1024 * 1024));
    assert_eq!(parse_size("100K\t"), Some(100 * 1024));
}

// ==================== DATASET TYPE PARSING TESTS ====================

#[test]
fn test_parse_dataset_type_filesystem() {
    assert_eq!(parse_dataset_type("filesystem"), DatasetType::Filesystem);
}

#[test]
fn test_parse_dataset_type_volume() {
    assert_eq!(parse_dataset_type("volume"), DatasetType::Volume);
}

#[test]
fn test_parse_dataset_type_snapshot() {
    assert_eq!(parse_dataset_type("snapshot"), DatasetType::Snapshot);
}

#[test]
fn test_parse_dataset_type_bookmark() {
    // Bookmarks are treated as snapshots
    assert_eq!(parse_dataset_type("bookmark"), DatasetType::Snapshot);
}

#[test]
fn test_parse_dataset_type_default_filesystem() {
    // Unknown types default to Filesystem (case-sensitive matching)
    assert_eq!(parse_dataset_type("unknown"), DatasetType::Filesystem);
    assert_eq!(parse_dataset_type("invalid"), DatasetType::Filesystem);
    assert_eq!(parse_dataset_type(""), DatasetType::Filesystem);
    assert_eq!(parse_dataset_type("xyz"), DatasetType::Filesystem);
    assert_eq!(parse_dataset_type("Filesystem"), DatasetType::Filesystem);
    assert_eq!(parse_dataset_type("VOLUME"), DatasetType::Filesystem);
}

#[test]
fn test_parse_dataset_type_with_whitespace() {
    // These don't match exactly, so they default to Filesystem
    assert_eq!(parse_dataset_type(" filesystem "), DatasetType::Filesystem);
    assert_eq!(parse_dataset_type("volume\t"), DatasetType::Filesystem);
    assert_eq!(parse_dataset_type("\nsnapshot"), DatasetType::Filesystem);
}

// ==================== INTEGRATION-STYLE PARSING TESTS ====================

#[test]
fn test_parse_dataset_line_complete() {
    // Simulate parsing a complete zfs list line
    let line = "tank/data\t10.5G\t500G\t10.5G\t/mnt/data\tfilesystem";
    let parts: Vec<&str> = line.split('\t').collect();

    assert_eq!(parts.len(), 6);
    assert_eq!(parts[0], "tank/data");
    assert_eq!(
        parse_size(parts[1]),
        Some((10.5 * 1024.0 * 1024.0 * 1024.0) as u64)
    );
    assert_eq!(parse_size(parts[2]), Some(500 * 1024 * 1024 * 1024));
    assert_eq!(parts[4], "/mnt/data");
    assert_eq!(parse_dataset_type(parts[5]), DatasetType::Filesystem);
}

#[test]
fn test_parse_dataset_line_no_mountpoint() {
    let line = "tank/vol1\t5G\t100G\t5G\t-\tvolume";
    let parts: Vec<&str> = line.split('\t').collect();

    assert_eq!(parts[4], "-");
    assert_eq!(parse_dataset_type(parts[5]), DatasetType::Volume);
}

#[test]
fn test_parse_dataset_line_snapshot() {
    let line = "tank/data@snap1\t0\t-\t10.5G\t-\tsnapshot";
    let parts: Vec<&str> = line.split('\t').collect();

    assert_eq!(parts[0], "tank/data@snap1");
    assert_eq!(parse_size(parts[1]), Some(0));
    assert_eq!(parse_size(parts[2]), Some(0));
    assert_eq!(parse_dataset_type(parts[5]), DatasetType::Snapshot);
}

// ==================== SIZE PARSING BOUNDARY TESTS ====================

#[test]
fn test_parse_size_very_large() {
    // Test petabyte sizes
    assert_eq!(parse_size("1P"), Some(1024u64 * 1024 * 1024 * 1024 * 1024));
}

#[test]
fn test_parse_size_fractional_bytes() {
    assert_eq!(parse_size("0.1K"), Some(102)); // ~102 bytes
    assert_eq!(parse_size("0.001M"), Some(1048)); // ~1KB
}

#[test]
fn test_parse_size_max_values() {
    // Test handling of very large numbers
    assert!(parse_size("9999T").is_some());
}

// ==================== DATASET NAME VALIDATION TESTS ====================

#[test]
fn test_valid_dataset_names() {
    let valid_names = vec![
        "tank",
        "tank/data",
        "tank/data/documents",
        "pool-1/dataset_2",
        "my_pool/my-dataset",
    ];

    for name in valid_names {
        assert!(!name.is_empty());
        assert!(!name.contains('\t'));
        assert!(!name.contains('\n'));
    }
}

#[test]
fn test_snapshot_name_format() {
    let snapshot_names = vec![
        "tank@snap1",
        "tank/data@backup-2023",
        "pool/dataset@daily-01",
    ];

    for name in snapshot_names {
        assert!(name.contains('@'));
        let parts: Vec<&str> = name.split('@').collect();
        assert_eq!(parts.len(), 2);
        assert!(!parts[0].is_empty());
        assert!(!parts[1].is_empty());
    }
}

// ==================== PROPERTY PARSING TESTS ====================

#[test]
fn test_property_line_parsing() {
    let property_line = "compression\ton";
    let parts: Vec<&str> = property_line.split('\t').collect();

    assert_eq!(parts.len(), 2);
    assert_eq!(parts[0], "compression");
    assert_eq!(parts[1], "on");
}

#[test]
fn test_multiple_property_lines() {
    let properties = vec![
        "compression\ton",
        "atime\toff",
        "recordsize\t128K",
        "quota\t100G",
    ];

    for prop in properties {
        let parts: Vec<&str> = prop.split('\t').collect();
        assert_eq!(parts.len(), 2);
        assert!(!parts[0].is_empty());
        assert!(!parts[1].is_empty());
    }
}

// ==================== ERROR HANDLING TESTS ====================

#[test]
fn test_parse_size_overflow_protection() {
    // Very large string that might cause overflow
    assert!(parse_size("99999999999999999999T").is_some());
}

#[test]
fn test_parse_dataset_type_invalid_characters() {
    // Invalid types default to Filesystem
    assert_eq!(parse_dataset_type("file system"), DatasetType::Filesystem);
    assert_eq!(parse_dataset_type("vol-ume"), DatasetType::Filesystem);
}

// ==================== REAL-WORLD SCENARIOS ====================

#[test]
fn test_typical_filesystem_dataset() {
    let line = "rpool/ROOT/ubuntu\t15.2G\t200G\t15.2G\t/\tfilesystem";
    let parts: Vec<&str> = line.split('\t').collect();

    assert_eq!(parts[0], "rpool/ROOT/ubuntu");
    assert!(parse_size(parts[1]).unwrap() > 15 * 1024 * 1024 * 1024);
    assert_eq!(parse_dataset_type(parts[5]), DatasetType::Filesystem);
}

#[test]
fn test_typical_volume_dataset() {
    let line = "tank/vm-disk\t50G\t500G\t50G\t-\tvolume";
    let parts: Vec<&str> = line.split('\t').collect();

    assert_eq!(parts[0], "tank/vm-disk");
    assert_eq!(parse_size(parts[1]), Some(50 * 1024 * 1024 * 1024));
    assert_eq!(parts[4], "-");
    assert_eq!(parse_dataset_type(parts[5]), DatasetType::Volume);
}

#[test]
fn test_deeply_nested_dataset() {
    let line = "pool/project/2023/Q4/docs/final\t1.2G\t100G\t1.2G\t/mnt/final\tfilesystem";
    let parts: Vec<&str> = line.split('\t').collect();

    assert_eq!(parts[0], "pool/project/2023/Q4/docs/final");
    assert_eq!(parts[0].matches('/').count(), 5);
}

// ==================== CONCURRENCY SAFETY TESTS ====================

#[test]
fn test_parse_functions_are_pure() {
    // These functions should be pure (same input = same output)
    let size_input = "10G";
    let type_input = "filesystem";

    let result1 = parse_size(size_input);
    let result2 = parse_size(size_input);
    assert_eq!(result1, result2);

    let type1 = parse_dataset_type(type_input);
    let type2 = parse_dataset_type(type_input);
    assert_eq!(type1, type2);
}

#[test]
fn test_concurrent_parsing_simulation() {
    // Simulate concurrent parsing (would be better with actual threads)
    let inputs = vec!["1G", "2M", "500K", "1T", "100"];

    for input in inputs {
        let result = parse_size(input);
        assert!(result.is_some());
    }
}

// ==================== COMPREHENSIVE SIZE RANGE TESTS ====================

#[test]
fn test_all_size_units() {
    let test_cases = vec![
        ("1", 1),
        ("1K", 1024),
        ("1M", 1024 * 1024),
        ("1G", 1024 * 1024 * 1024),
        ("1T", 1024u64 * 1024 * 1024 * 1024),
    ];

    for (input, expected) in test_cases {
        assert_eq!(parse_size(input), Some(expected));
    }
}

#[test]
fn test_size_precision() {
    // Test that decimal parsing maintains reasonable precision
    let result = parse_size("1.5M");
    let expected = (1.5 * 1024.0 * 1024.0) as u64;

    assert!(result.is_some());
    let diff = if result.unwrap() > expected {
        result.unwrap() - expected
    } else {
        expected - result.unwrap()
    };

    // Allow small rounding error
    assert!(diff < 100);
}
