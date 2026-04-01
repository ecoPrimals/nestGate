// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Unit tests for native ZFS snapshot operations
//!
//! These tests cover snapshot parsing, size parsing, and data structure validation
//! without requiring actual ZFS installation.

#[cfg(test)]
mod tests {
    use super::super::snapshot_operations::*;
    use crate::handlers::zfs::universal_zfs_types::{SnapshotConfig, SnapshotInfo};

    // ==================== parse_size Tests ====================

    #[test]
    fn test_parse_size_dash() {
        let result = parse_size("-");
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_parse_size_empty() {
        let result = parse_size("");
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_size_bytes() {
        let result = parse_size("1024");
        assert_eq!(result, Some(1024));
    }

    #[test]
    fn test_parse_size_kilobytes() {
        let result = parse_size("10K");
        assert_eq!(result, Some(10 * 1024));
    }

    #[test]
    fn test_parse_size_megabytes() {
        let result = parse_size("5M");
        assert_eq!(result, Some(5 * 1024 * 1024));
    }

    #[test]
    fn test_parse_size_gigabytes() {
        let result = parse_size("2G");
        assert_eq!(result, Some(2 * 1024 * 1024 * 1024));
    }

    #[test]
    fn test_parse_size_terabytes() {
        let result = parse_size("1T");
        assert_eq!(result, Some(1024u64 * 1024 * 1024 * 1024));
    }

    #[test]
    fn test_parse_size_petabytes() {
        let result = parse_size("1P");
        assert_eq!(result, Some(1024u64 * 1024 * 1024 * 1024 * 1024));
    }

    #[test]
    fn test_parse_size_lowercase_k() {
        let result = parse_size("10k");
        assert_eq!(result, Some(10 * 1024));
    }

    #[test]
    fn test_parse_size_lowercase_m() {
        let result = parse_size("5m");
        assert_eq!(result, Some(5 * 1024 * 1024));
    }

    #[test]
    fn test_parse_size_lowercase_g() {
        let result = parse_size("2g");
        assert_eq!(result, Some(2 * 1024 * 1024 * 1024));
    }

    #[test]
    fn test_parse_size_lowercase_t() {
        let result = parse_size("1t");
        assert_eq!(result, Some(1024u64 * 1024 * 1024 * 1024));
    }

    #[test]
    fn test_parse_size_lowercase_p() {
        let result = parse_size("1p");
        assert_eq!(result, Some(1024u64 * 1024 * 1024 * 1024 * 1024));
    }

    #[test]
    fn test_parse_size_with_whitespace() {
        let result = parse_size("  10K  ");
        assert_eq!(result, Some(10 * 1024));
    }

    #[test]
    fn test_parse_size_decimal() {
        let result = parse_size("1.5G");
        // Should handle decimal parsing
        assert!(result.is_some());
    }

    #[test]
    fn test_parse_size_large_number() {
        let result = parse_size("999G");
        assert_eq!(result, Some(999 * 1024 * 1024 * 1024));
    }

    #[test]
    fn test_parse_size_zero() {
        let result = parse_size("0");
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_parse_size_zero_with_unit() {
        let result = parse_size("0K");
        assert_eq!(result, Some(0));
    }

    // ==================== parse_snapshot_line Tests ====================

    #[test]
    fn test_parse_snapshot_line_valid() {
        let line = "tank/data@snap1\t10M\t1699900000";
        let result = parse_snapshot_line(line);
        
        assert!(result.is_some());
        let snapshot = result.unwrap();
        assert_eq!(snapshot.name, "tank/data@snap1");
        assert_eq!(snapshot.dataset, "tank/data");
        assert_eq!(snapshot.size_bytes, 10 * 1024 * 1024);
    }

    #[test]
    fn test_parse_snapshot_line_with_dash_size() {
        let line = "tank/data@snap1\t-\t1699900000";
        let result = parse_snapshot_line(line);
        
        assert!(result.is_some());
        let snapshot = result.unwrap();
        assert_eq!(snapshot.size_bytes, 0);
    }

    #[test]
    fn test_parse_snapshot_line_empty() {
        let line = "";
        let result = parse_snapshot_line(line);
        
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_snapshot_line_insufficient_fields() {
        let line = "tank/data@snap1\t10M";
        let result = parse_snapshot_line(line);
        
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_snapshot_line_with_extra_fields() {
        let line = "tank/data@snap1\t10M\t1699900000\textra\tfields";
        let result = parse_snapshot_line(line);
        
        assert!(result.is_some());
        let snapshot = result.unwrap();
        assert_eq!(snapshot.name, "tank/data@snap1");
    }

    #[test]
    fn test_parse_snapshot_line_nested_dataset() {
        let line = "tank/data/users/john@backup\t5G\t1699900000";
        let result = parse_snapshot_line(line);
        
        assert!(result.is_some());
        let snapshot = result.unwrap();
        assert_eq!(snapshot.name, "tank/data/users/john@backup");
        assert_eq!(snapshot.dataset, "tank/data/users/john");
    }

    #[test]
    fn test_parse_snapshot_line_root_dataset() {
        let line = "tank@snap1\t1G\t1699900000";
        let result = parse_snapshot_line(line);
        
        assert!(result.is_some());
        let snapshot = result.unwrap();
        assert_eq!(snapshot.dataset, "tank");
    }

    // ==================== SnapshotConfig Tests ====================

    #[test]
    fn test_snapshot_config_creation() {
        let config = SnapshotConfig {
            name: "backup-2024".to_string(),
            dataset: "tank/data".to_string(),
            recursive: false,
            properties: std::collections::HashMap::new(),
        };

        assert_eq!(config.name, "backup-2024");
        assert_eq!(config.dataset, "tank/data");
        assert!(!config.recursive);
    }

    #[test]
    fn test_snapshot_config_with_properties() {
        let mut properties = std::collections::HashMap::new();
        properties.insert("compression".to_string(), "lz4".to_string());
        
        let config = SnapshotConfig {
            name: "backup".to_string(),
            dataset: "tank".to_string(),
            recursive: true,
            properties,
        };

        assert_eq!(config.properties.len(), 1);
        assert_eq!(config.properties.get("compression"), Some(&"lz4".to_string()));
    }

    #[test]
    fn test_snapshot_config_clone() {
        let config1 = SnapshotConfig {
            name: "snap1".to_string(),
            dataset: "tank".to_string(),
            recursive: false,
            properties: std::collections::HashMap::new(),
        };

        let config2 = config1.clone();
        assert_eq!(config1.name, config2.name);
        assert_eq!(config1.dataset, config2.dataset);
    }

    #[test]
    fn test_snapshot_config_serialization() {
        let config = SnapshotConfig {
            name: "test".to_string(),
            dataset: "tank".to_string(),
            recursive: false,
            properties: std::collections::HashMap::new(),
        };

        let json = serde_json::to_string(&config).expect("Failed to serialize");
        assert!(json.contains("test"));
        assert!(json.contains("tank"));
    }

    // ==================== SnapshotInfo Tests ====================

    #[test]
    fn test_snapshot_info_creation() {
        let info = SnapshotInfo {
            name: "tank@snap1".to_string(),
            dataset: "tank".to_string(),
            created_at: std::time::SystemTime::now(),
            size_bytes: 1024 * 1024,
            properties: std::collections::HashMap::new(),
            description: Some("Test snapshot".to_string()),
        };

        assert_eq!(info.name, "tank@snap1");
        assert_eq!(info.dataset, "tank");
        assert_eq!(info.size_bytes, 1024 * 1024);
        assert_eq!(info.description, Some("Test snapshot".to_string()));
    }

    #[test]
    fn test_snapshot_info_without_description() {
        let info = SnapshotInfo {
            name: "tank@snap1".to_string(),
            dataset: "tank".to_string(),
            created_at: std::time::SystemTime::now(),
            size_bytes: 0,
            properties: std::collections::HashMap::new(),
            description: None,
        };

        assert!(info.description.is_none());
    }

    #[test]
    fn test_snapshot_info_clone() {
        let info1 = SnapshotInfo {
            name: "tank@snap1".to_string(),
            dataset: "tank".to_string(),
            created_at: std::time::SystemTime::now(),
            size_bytes: 1024,
            properties: std::collections::HashMap::new(),
            description: None,
        };

        let info2 = info1.clone();
        assert_eq!(info1.name, info2.name);
        assert_eq!(info1.size_bytes, info2.size_bytes);
    }

    #[test]
    fn test_snapshot_info_serialization() {
        let info = SnapshotInfo {
            name: "tank@snap1".to_string(),
            dataset: "tank".to_string(),
            created_at: std::time::SystemTime::now(),
            size_bytes: 1024,
            properties: std::collections::HashMap::new(),
            description: None,
        };

        let json = serde_json::to_string(&info).expect("Failed to serialize");
        assert!(json.contains("tank@snap1"));
    }

    // ==================== Edge Cases ====================

    #[test]
    fn test_parse_size_invalid_unit() {
        let result = parse_size("10X");
        // Should parse as 10 bytes (unknown unit defaults to 1)
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_parse_size_no_number() {
        let result = parse_size("K");
        // Should fail to parse
        assert!(result.is_none() || result == Some(0));
    }

    #[test]
    fn test_parse_snapshot_line_malformed() {
        let line = "not-a-valid-snapshot-line";
        let result = parse_snapshot_line(line);
        
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_snapshot_line_no_at_symbol() {
        let line = "tank/data-no-snapshot\t10M\t1699900000";
        let result = parse_snapshot_line(line);
        
        // Should still parse, dataset extraction handles missing @
        assert!(result.is_some());
    }

    #[test]
    fn test_snapshot_config_empty_name() {
        let config = SnapshotConfig {
            name: String::new(),
            dataset: "tank".to_string(),
            recursive: false,
            properties: std::collections::HashMap::new(),
        };

        assert!(config.name.is_empty());
    }

    #[test]
    fn test_snapshot_config_recursive() {
        let config = SnapshotConfig {
            name: "recursive-snap".to_string(),
            dataset: "tank/data".to_string(),
            recursive: true,
            properties: std::collections::HashMap::new(),
        };

        assert!(config.recursive);
    }

    #[test]
    fn test_snapshot_info_large_size() {
        let info = SnapshotInfo {
            name: "tank@large".to_string(),
            dataset: "tank".to_string(),
            created_at: std::time::SystemTime::now(),
            size_bytes: 1024u64 * 1024 * 1024 * 1024 * 10, // 10 TB
            properties: std::collections::HashMap::new(),
            description: None,
        };

        assert_eq!(info.size_bytes, 1024u64 * 1024 * 1024 * 1024 * 10);
    }

    #[test]
    fn test_snapshot_info_zero_size() {
        let info = SnapshotInfo {
            name: "tank@empty".to_string(),
            dataset: "tank".to_string(),
            created_at: std::time::SystemTime::now(),
            size_bytes: 0,
            properties: std::collections::HashMap::new(),
            description: None,
        };

        assert_eq!(info.size_bytes, 0);
    }

    // ==================== Integration Scenarios ====================

    #[test]
    fn test_parse_multiple_snapshot_lines() {
        let lines = vec![
            "tank/data@snap1\t10M\t1699900000",
            "tank/data@snap2\t20M\t1699900100",
            "tank/data@snap3\t30M\t1699900200",
        ];

        let snapshots: Vec<_> = lines.iter()
            .filter_map(|line| parse_snapshot_line(line))
            .collect();

        assert_eq!(snapshots.len(), 3);
        assert_eq!(snapshots[0].name, "tank/data@snap1");
        assert_eq!(snapshots[1].name, "tank/data@snap2");
        assert_eq!(snapshots[2].name, "tank/data@snap3");
    }

    #[test]
    fn test_parse_mixed_valid_invalid_lines() {
        let lines = vec![
            "tank/data@snap1\t10M\t1699900000",
            "invalid line",
            "tank/data@snap2\t20M\t1699900100",
            "",
            "tank/data@snap3\t30M\t1699900200",
        ];

        let snapshots: Vec<_> = lines.iter()
            .filter_map(|line| parse_snapshot_line(line))
            .collect();

        // Only 3 valid snapshots should be parsed
        assert_eq!(snapshots.len(), 3);
    }

    #[test]
    fn test_snapshot_name_formatting() {
        let config = SnapshotConfig {
            name: "backup".to_string(),
            dataset: "tank/data".to_string(),
            recursive: false,
            properties: std::collections::HashMap::new(),
        };

        // Test expected snapshot name format
        let expected_name = format!("{}@{}", config.dataset, config.name);
        assert_eq!(expected_name, "tank/data@backup");
    }

    #[test]
    fn test_snapshot_name_with_at_symbol() {
        let config = SnapshotConfig {
            name: "tank/data@backup".to_string(),
            dataset: "tank/data".to_string(),
            recursive: false,
            properties: std::collections::HashMap::new(),
        };

        // Name already contains @ symbol
        assert!(config.name.contains('@'));
    }
}

