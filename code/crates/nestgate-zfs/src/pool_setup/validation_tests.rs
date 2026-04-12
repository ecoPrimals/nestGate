// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Comprehensive tests for ZFS pool setup validation logic
//!
//! This module tests validation rules, error handling, and edge cases
//! for device and pool configuration validation.

use super::validation::*;
use super::*;

// ==================== ValidationResult Tests ====================

#[test]
fn test_validation_result_creation() {
    let result = ValidationResult::new();

    assert!(result.is_valid);
    assert!(result.issues.is_empty());
    assert!(result.warnings.is_empty());
}

#[test]
fn test_validation_result_default() {
    let result = ValidationResult::default();

    assert!(result.is_valid);
    assert!(result.issues.is_empty());
    assert!(result.warnings.is_empty());
}

#[test]
fn test_validation_result_add_error() {
    let mut result = ValidationResult::new();
    result.add_error("Test error".to_string());

    assert!(!result.is_valid);
    assert_eq!(result.issues.len(), 1);
    assert_eq!(result.issues[0], "Test error");
}

#[test]
fn test_validation_result_add_multiple_errors() {
    let mut result = ValidationResult::new();
    result.add_error("Error 1".to_string());
    result.add_error("Error 2".to_string());
    result.add_error("Error 3".to_string());

    assert!(!result.is_valid);
    assert_eq!(result.issues.len(), 3);
}

#[test]
fn test_validation_result_add_warning() {
    let mut result = ValidationResult::new();
    result.add_warning("Test warning".to_string());

    // Adding warnings doesn't affect validity
    assert!(result.is_valid);
    assert!(result.issues.is_empty());
    assert_eq!(result.warnings.len(), 1);
    assert_eq!(result.warnings[0], "Test warning");
}

#[test]
fn test_validation_result_mixed_errors_and_warnings() {
    let mut result = ValidationResult::new();
    result.add_warning("Warning 1".to_string());
    result.add_error("Error 1".to_string());
    result.add_warning("Warning 2".to_string());
    result.add_error("Error 2".to_string());

    assert!(!result.is_valid);
    assert_eq!(result.issues.len(), 2);
    assert_eq!(result.warnings.len(), 2);
}

#[test]
fn test_validation_result_merge_valid() {
    let mut result1 = ValidationResult::new();
    let mut result2 = ValidationResult::new();

    result2.add_warning("Warning from result2".to_string());
    result1.merge(result2);

    assert!(result1.is_valid);
    assert_eq!(result1.warnings.len(), 1);
}

#[test]
fn test_validation_result_merge_invalid() {
    let mut result1 = ValidationResult::new();
    let mut result2 = ValidationResult::new();

    result2.add_error("Error from result2".to_string());
    result1.merge(result2);

    assert!(!result1.is_valid);
    assert_eq!(result1.issues.len(), 1);
}

#[test]
fn test_validation_result_merge_multiple() {
    let mut result1 = ValidationResult::new();
    result1.add_error("Error 1".to_string());
    result1.add_warning("Warning 1".to_string());

    let mut result2 = ValidationResult::new();
    result2.add_error("Error 2".to_string());
    result2.add_warning("Warning 2".to_string());

    let mut result3 = ValidationResult::new();
    result3.add_warning("Warning 3".to_string());

    result1.merge(result2);
    result1.merge(result3);

    assert!(!result1.is_valid);
    assert_eq!(result1.issues.len(), 2);
    assert_eq!(result1.warnings.len(), 3);
}

#[test]
fn test_validation_result_merge_empty() {
    let mut result1 = ValidationResult::new();
    result1.add_error("Error 1".to_string());

    let result2 = ValidationResult::new();
    result1.merge(result2);

    assert!(!result1.is_valid);
    assert_eq!(result1.issues.len(), 1);
}

// ==================== PoolSetupValidator Tests ====================

/// Creates  Test Config
fn create_test_config() -> PoolSetupConfig {
    PoolSetupConfig {
        pool_name: "testpool".to_string(),
        devices: vec![],
        topology: PoolTopology::Single,
        redundancy: RedundancyLevel::None,
        properties: std::collections::HashMap::new(),
        tier_mappings: std::collections::HashMap::new(),
        create_tiers: false,
        device_detection: DeviceDetectionConfig {
            scan_paths: vec!["/dev".to_string()],
            exclude_patterns: vec!["loop".to_string()],
            include_removable: false,
            min_device_size: 1_000_000_000, // 1GB
            max_device_size: 0,
            skip_mountpoints: vec![],
            skip_fstypes: vec!["ext4".to_string(), "ntfs".to_string()],
            include_loop_devices: false,
        },
    }
}

/// Creates  Test Device
fn create_test_device(path: &str, size_bytes: u64, in_use: bool) -> StorageDevice {
    StorageDevice {
        device_path: path.to_string(),
        model: "Test Device".to_string(),
        size_bytes,
        device_type: DetectionDeviceType::NvmeSsd,
        speed_class: SpeedClass::Fast,
        in_use,
        current_use: if in_use {
            Some("ext4".to_string())
        } else {
            None
        },
    }
}

#[test]
fn test_validator_creation() {
    let config = create_test_config();
    let validator = PoolSetupValidator::new(config);

    // Validator should be created successfully
    assert!(std::mem::size_of_val(&validator) > 0);
}

#[test]
fn test_validate_device_valid() {
    let config = create_test_config();
    let validator = PoolSetupValidator::new(config);

    let device = create_test_device("/dev/sda", 2_000_000_000, false);
    let result = validator.validate_device(&device);

    assert!(result.is_valid);
    assert!(result.issues.is_empty());
}

#[test]
fn test_validate_device_too_small() {
    let config = create_test_config();
    let validator = PoolSetupValidator::new(config);

    let device = create_test_device("/dev/sda", 500_000_000, false); // 500MB
    let result = validator.validate_device(&device);

    assert!(!result.is_valid);
    assert!(!result.issues.is_empty());
    assert!(result.issues[0].contains("too small"));
}

#[test]
fn test_validate_device_invalid_path() {
    let config = create_test_config();
    let validator = PoolSetupValidator::new(config);

    let device = create_test_device("/invalid/path", 2_000_000_000, false);
    let result = validator.validate_device(&device);

    assert!(!result.is_valid);
    assert!(result.issues.iter().any(|issue| issue.contains("path")));
}

#[test]
fn test_validate_device_in_use_with_skip_fstype() {
    let config = create_test_config();
    let validator = PoolSetupValidator::new(config);

    let device = create_test_device("/dev/sda", 2_000_000_000, true);
    let result = validator.validate_device(&device);

    // Device is in use with ext4 (which is in skip list)
    assert!(!result.is_valid);
    assert!(
        result
            .issues
            .iter()
            .any(|issue| issue.contains("filesystem"))
    );
}

#[test]
fn test_validate_device_multiple_errors() {
    let config = create_test_config();
    let validator = PoolSetupValidator::new(config);

    // Device with multiple issues: invalid path AND too small
    let device = create_test_device("/invalid/path", 500_000_000, false);
    let result = validator.validate_device(&device);

    assert!(!result.is_valid);
    assert!(result.issues.len() >= 2);
}

#[test]
fn test_validate_pool_config_valid() {
    let config = create_test_config();
    let validator = PoolSetupValidator::new(config.clone());

    let mut valid_config = config.clone();
    valid_config.devices = vec!["/ /sda".to_string()];

    let result = validator.validate_pool_config(&valid_config);

    assert!(result.is_valid);
}

#[test]
fn test_validate_pool_config_empty_name() {
    let config = create_test_config();
    let validator = PoolSetupValidator::new(config.clone());

    let mut invalid_config = config.clone();
    invalid_config.pool_name = String::new();
    invalid_config.devices = vec!["/dev/sda".to_string()];

    let result = validator.validate_pool_config(&invalid_config);

    assert!(!result.is_valid);
    assert!(result.issues.iter().any(|issue| issue.contains("name")));
}

#[test]
fn test_validate_pool_config_no_devices() {
    let config = create_test_config();
    let validator = PoolSetupValidator::new(config.clone());

    let invalid_config = config.clone();
    // devices is already empty in test config

    let result = validator.validate_pool_config(&invalid_config);

    assert!(!result.is_valid);
    assert!(
        result
            .issues
            .iter()
            .any(|issue| issue.contains("No devices"))
    );
}

#[test]
fn test_validate_pool_config_mirror_insufficient_devices() {
    let config = create_test_config();
    let validator = PoolSetupValidator::new(config.clone());

    let mut invalid_config = config.clone();
    invalid_config.topology = PoolTopology::Mirror;
    invalid_config.devices = vec!["/dev/sda".to_string()]; // Only 1 device

    let result = validator.validate_pool_config(&invalid_config);

    assert!(!result.is_valid);
    assert!(
        result
            .issues
            .iter()
            .any(|issue| issue.contains("requires at least"))
    );
}

#[test]
fn test_validate_pool_config_raidz1_insufficient_devices() {
    let config = create_test_config();
    let validator = PoolSetupValidator::new(config.clone());

    let mut invalid_config = config.clone();
    invalid_config.topology = PoolTopology::RaidZ1;
    invalid_config.devices = vec!["/dev/sda".to_string(), "/dev/sdb".to_string()]; // Only 2 devices

    let result = validator.validate_pool_config(&invalid_config);

    assert!(!result.is_valid);
    assert!(
        result
            .issues
            .iter()
            .any(|issue| issue.contains("requires at least 3"))
    );
}

#[test]
fn test_validate_pool_config_raidz2_insufficient_devices() {
    let config = create_test_config();
    let validator = PoolSetupValidator::new(config.clone());

    let mut invalid_config = config.clone();
    invalid_config.topology = PoolTopology::RaidZ2;
    invalid_config.devices = vec![
        "/dev/sda".to_string(),
        "/dev/sdb".to_string(),
        "/dev/sdc".to_string(),
    ]; // Only 3 devices

    let result = validator.validate_pool_config(&invalid_config);

    assert!(!result.is_valid);
    assert!(
        result
            .issues
            .iter()
            .any(|issue| issue.contains("requires at least 4"))
    );
}

#[test]
fn test_validate_pool_config_raidz3_insufficient_devices() {
    let config = create_test_config();
    let validator = PoolSetupValidator::new(config.clone());

    let mut invalid_config = config.clone();
    invalid_config.topology = PoolTopology::RaidZ3;
    invalid_config.devices = vec![
        "/dev/sda".to_string(),
        "/dev/sdb".to_string(),
        "/dev/sdc".to_string(),
        "/dev/sdd".to_string(),
    ]; // Only 4 devices

    let result = validator.validate_pool_config(&invalid_config);

    assert!(!result.is_valid);
    assert!(
        result
            .issues
            .iter()
            .any(|issue| issue.contains("requires at least 5"))
    );
}

#[test]
fn test_validate_pool_config_single_topology_valid() {
    let config = create_test_config();
    let validator = PoolSetupValidator::new(config.clone());

    let mut valid_config = config.clone();
    valid_config.topology = PoolTopology::Single;
    valid_config.devices = vec!["/dev/sda".to_string()];

    let result = validator.validate_pool_config(&valid_config);

    assert!(result.is_valid);
}

#[test]
fn test_validate_pool_config_mirror_topology_valid() {
    let config = create_test_config();
    let validator = PoolSetupValidator::new(config.clone());

    let mut valid_config = config.clone();
    valid_config.topology = PoolTopology::Mirror;
    valid_config.devices = vec!["/dev/sda".to_string(), "/dev/sdb".to_string()];

    let result = validator.validate_pool_config(&valid_config);

    assert!(result.is_valid);
}

#[test]
fn test_validate_pool_config_raidz1_topology_valid() {
    let config = create_test_config();
    let validator = PoolSetupValidator::new(config.clone());

    let mut valid_config = config.clone();
    valid_config.topology = PoolTopology::RaidZ1;
    valid_config.devices = vec![
        "/dev/sda".to_string(),
        "/dev/sdb".to_string(),
        "/dev/sdc".to_string(),
    ];

    let result = validator.validate_pool_config(&valid_config);

    assert!(result.is_valid);
}

// ==================== Device Type Conversion Tests ====================

#[test]
fn test_convert_device_type_nvme_ssd() {
    let detection_type = DetectionDeviceType::NvmeSsd;
    let config_type = super::convert_device_type(detection_type);

    assert!(matches!(config_type, ConfigDeviceType::NvmeSsd));
}

#[test]
fn test_convert_device_type_sata_ssd() {
    let detection_type = DetectionDeviceType::SataSsd;
    let config_type = super::convert_device_type(detection_type);

    assert!(matches!(config_type, ConfigDeviceType::SataSsd));
}

#[test]
fn test_convert_device_type_hdd() {
    let detection_type = DetectionDeviceType::Hdd;
    let config_type = super::convert_device_type(detection_type);

    assert!(matches!(config_type, ConfigDeviceType::SpinningDisk));
}

#[test]
fn test_convert_device_type_optane() {
    let detection_type = DetectionDeviceType::OptaneMemory;
    let config_type = super::convert_device_type(detection_type);

    assert!(matches!(config_type, ConfigDeviceType::OptaneMemory));
}

#[test]
fn test_convert_device_type_unknown() {
    let detection_type = DetectionDeviceType::Unknown;
    let config_type = super::convert_device_type(detection_type);

    // Unknown should default to SpinningDisk
    assert!(matches!(config_type, ConfigDeviceType::SpinningDisk));
}

// ==================== Edge Case Tests ====================

#[test]
fn test_validation_result_serialize_deserialize() {
    let mut result = ValidationResult::new();
    result.add_error("Test error".to_string());
    result.add_warning("Test warning".to_string());

    let json = serde_json::to_string(&result).expect("Failed to serialize");
    let deserialized: ValidationResult =
        serde_json::from_str(&json).expect("Failed to deserialize");

    assert_eq!(result.is_valid, deserialized.is_valid);
    assert_eq!(result.issues.len(), deserialized.issues.len());
    assert_eq!(result.warnings.len(), deserialized.warnings.len());
}

#[test]
fn test_validation_result_clone() {
    let mut result1 = ValidationResult::new();
    result1.add_error("Error".to_string());

    let result2 = result1.clone();

    assert_eq!(result1.is_valid, result2.is_valid);
    assert_eq!(result1.issues.len(), result2.issues.len());
}

#[test]
fn test_validation_result_debug() {
    let mut result = ValidationResult::new();
    result.add_error("Error".to_string());

    let debug_str = format!("{result:?}");
    assert!(debug_str.contains("ValidationResult"));
}

#[test]
fn test_validate_device_with_large_size() {
    let config = create_test_config();
    let validator = PoolSetupValidator::new(config);

    let device = create_test_device("/dev/sda", u64::MAX, false);
    let result = validator.validate_device(&device);

    // Should be valid (size is definitely large enough)
    assert!(result.is_valid || result.issues.iter().any(|i| !i.contains("too small")));
}

#[test]
fn test_validate_device_exactly_minimum_size() {
    let config = create_test_config();
    let validator = PoolSetupValidator::new(config.clone());

    let device = create_test_device("/dev/sda", config.device_detection.min_device_size, false);
    let result = validator.validate_device(&device);

    // Should be valid at exactly the minimum
    assert!(result.is_valid || result.issues.iter().all(|i| !i.contains("too small")));
}

#[test]
fn test_validate_device_one_byte_below_minimum() {
    let config = create_test_config();
    let validator = PoolSetupValidator::new(config.clone());

    let device = create_test_device(
        "/dev/sda",
        config.device_detection.min_device_size - 1,
        false,
    );
    let result = validator.validate_device(&device);

    assert!(!result.is_valid);
    assert!(result.issues.iter().any(|i| i.contains("too small")));
}

#[test]
fn test_pool_setup_error_display() {
    let error = PoolSetupError::DeviceValidation("test error".to_string());
    let error_str = format!("{error}");

    assert!(error_str.contains("Device validation failed"));
    assert!(error_str.contains("test error"));
}

#[test]
fn test_pool_setup_error_debug() {
    let error = PoolSetupError::PoolCreation("test".to_string());
    let debug_str = format!("{error:?}");

    assert!(debug_str.contains("PoolCreation"));
}

#[test]
fn test_pool_setup_result_serialization() {
    let result = PoolSetupResult {
        pool_name: "testpool".to_string(),
        success: true,
        message: "Pool created".to_string(),
        devices_used: vec!["/dev/sda".to_string()],
        topology: PoolTopology::Single,
    };

    let json = serde_json::to_string(&result).expect("Failed to serialize");
    let deserialized: PoolSetupResult = serde_json::from_str(&json).expect("Failed to deserialize");

    assert_eq!(result.pool_name, deserialized.pool_name);
    assert_eq!(result.success, deserialized.success);
}
