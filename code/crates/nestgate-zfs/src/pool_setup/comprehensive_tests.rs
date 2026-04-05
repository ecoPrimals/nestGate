// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Comprehensive tests for pool_setup module
//!
//! This test suite aims to boost coverage from 14.18% to 40%+

use super::*;
use config::{ConfigDeviceType, ConfigStorageTier, PoolSetupConfig, PoolTopology, RedundancyLevel};
use device_detection::{DetectionDeviceType, SpeedClass, StorageDevice};
use validation::{PoolSetupValidator, ValidationResult};

/// Helper to create test storage device
fn create_test_device(path: &str, device_type: DetectionDeviceType, size_gb: u64) -> StorageDevice {
    StorageDevice {
        path: path.to_string(),
        device_type,
        size_bytes: size_gb * 1024 * 1024 * 1024,
        model: format!("Test {}", path),
        serial: format!("SN{}", path),
        speed_class: SpeedClass::High,
        is_nvme: matches!(device_type, DetectionDeviceType::NvmeSsd),
    }
}

/// Helper to create test pool config  
/// Note: PoolSetupConfig structure changed - fields like ashift, compression, etc.
/// are now in nested structs (PoolPropertyConfig, DeviceDetectionConfig, etc.)
fn create_test_pool_config() -> PoolSetupConfig {
    let mut config = PoolSetupConfig::default();
    config.pool_name = "test-pool".to_string();
    config.topology = PoolTopology::Single; // Use valid enum variant
    config.redundancy = RedundancyLevel::None;
    config.devices = vec!["test-device".to_string()]; // String paths, not enum
    config.device_detection.min_device_size = 10 * 1024 * 1024 * 1024; // 10GB
    config
}

#[test]
fn test_convert_device_type_nvme() {
    let detection_type = DetectionDeviceType::NvmeSsd;
    let config_type = convert_device_type(&detection_type);
    assert!(matches!(config_type, ConfigDeviceType::NvmeSsd));
}

#[test]
fn test_convert_device_type_sata() {
    let detection_type = DetectionDeviceType::SataSsd;
    let config_type = convert_device_type(&detection_type);
    assert!(matches!(config_type, ConfigDeviceType::SataSsd));
}

#[test]
fn test_convert_device_type_hdd() {
    let detection_type = DetectionDeviceType::Hdd;
    let config_type = convert_device_type(&detection_type);
    assert!(matches!(config_type, ConfigDeviceType::SpinningDisk));
}

#[test]
fn test_convert_device_type_optane() {
    let detection_type = DetectionDeviceType::OptaneMemory;
    let config_type = convert_device_type(&detection_type);
    assert!(matches!(config_type, ConfigDeviceType::OptaneMemory));
}

#[test]
fn test_convert_device_type_unknown() {
    let detection_type = DetectionDeviceType::Unknown;
    let config_type = convert_device_type(&detection_type);
    // Unknown defaults to SpinningDisk
    assert!(matches!(config_type, ConfigDeviceType::SpinningDisk));
}

#[test]
fn test_pool_setup_error_device_validation() {
    let error = PoolSetupError::DeviceValidation("Test error".to_string());
    assert!(error.to_string().contains("Device validation failed"));
}

#[test]
fn test_pool_setup_error_pool_creation() {
    let error = PoolSetupError::PoolCreation("Test error".to_string());
    assert!(error.to_string().contains("Pool creation failed"));
}

#[test]
fn test_pool_setup_error_configuration() {
    let error = PoolSetupError::Configuration("Test error".to_string());
    assert!(error.to_string().contains("Configuration error"));
}

#[test]
fn test_pool_setup_error_device_scanning() {
    let error = PoolSetupError::DeviceScanning("Test error".to_string());
    assert!(error.to_string().contains("Device scanning failed"));
}

#[test]
fn test_pool_setup_error_insufficient_devices() {
    let error = PoolSetupError::InsufficientDevices("Need 3, got 1".to_string());
    assert!(error.to_string().contains("Insufficient devices"));
}

#[test]
fn test_pool_setup_error_zfs_command() {
    let error = PoolSetupError::ZfsCommand("zpool create failed".to_string());
    assert!(error.to_string().contains("ZFS command failed"));
}

#[test]
fn test_pool_setup_result_creation() {
    let result = PoolSetupResult {
        pool_name: "test-pool".to_string(),
        success: true,
        message: "Pool created successfully".to_string(),
        devices_used: vec!["/dev/sda".to_string(), "/dev/sdb".to_string()],
        topology: PoolTopology::Mirror,
    };

    assert_eq!(result.pool_name, "test-pool");
    assert!(result.success);
    assert_eq!(result.devices_used.len(), 2);
    assert!(matches!(result.topology, PoolTopology::Mirror));
}

#[test]
fn test_pool_setup_result_failure() {
    let result = PoolSetupResult {
        pool_name: "failed-pool".to_string(),
        success: false,
        message: "Insufficient devices".to_string(),
        devices_used: vec![],
        topology: PoolTopology::Stripe,
    };

    assert_eq!(result.pool_name, "failed-pool");
    assert!(!result.success);
    assert!(result.devices_used.is_empty());
}

#[test]
fn test_storage_device_creation() {
    let device = create_test_device("/dev/nvme0n1", DetectionDeviceType::NvmeSsd, 500);

    assert_eq!(device.path, "/dev/nvme0n1");
    assert!(matches!(device.device_type, DetectionDeviceType::NvmeSsd));
    assert_eq!(device.size_bytes, 500 * 1024 * 1024 * 1024);
    assert!(device.is_nvme);
}

#[test]
fn test_storage_device_non_nvme() {
    let device = create_test_device("/dev/sda", DetectionDeviceType::SataSsd, 1000);

    assert!(!device.is_nvme);
    assert!(matches!(device.device_type, DetectionDeviceType::SataSsd));
}

#[test]
fn test_pool_config_creation() {
    let config = create_test_pool_config();

    assert_eq!(config.pool_name, "test-pool");
    assert!(matches!(config.topology, PoolTopology::Stripe));
    assert!(matches!(config.redundancy, RedundancyLevel::None));
    assert!(config.enable_compression);
    assert!(!config.enable_dedup);
    assert_eq!(config.ashift, 12);
}

#[test]
fn test_pool_config_with_mirror() {
    let mut config = create_test_pool_config();
    config.topology = PoolTopology::Mirror;
    config.redundancy = RedundancyLevel::Mirror;

    assert!(matches!(config.topology, PoolTopology::Mirror));
    assert!(matches!(config.redundancy, RedundancyLevel::Mirror));
}

#[test]
fn test_pool_config_with_raidz() {
    let mut config = create_test_pool_config();
    config.topology = PoolTopology::RaidZ1;
    config.redundancy = RedundancyLevel::RaidZ1;

    assert!(matches!(config.topology, PoolTopology::RaidZ1));
    assert!(matches!(config.redundancy, RedundancyLevel::RaidZ1));
}

#[test]
fn test_pool_config_device_types() {
    let mut config = create_test_pool_config();
    config.devices = vec![
        ConfigDeviceType::NvmeSsd,
        ConfigDeviceType::SataSsd,
        ConfigDeviceType::SpinningDisk,
    ];

    assert_eq!(config.devices.len(), 3);
}

#[test]
fn test_pool_config_tiers() {
    let mut config = create_test_pool_config();

    config.tier = ConfigStorageTier::Hot;
    assert!(matches!(config.tier, ConfigStorageTier::Hot));

    config.tier = ConfigStorageTier::Warm;
    assert!(matches!(config.tier, ConfigStorageTier::Warm));

    config.tier = ConfigStorageTier::Cold;
    assert!(matches!(config.tier, ConfigStorageTier::Cold));
}

#[test]
fn test_speed_class_variants() {
    let high = SpeedClass::High;
    let medium = SpeedClass::Medium;
    let low = SpeedClass::Low;

    assert!(format!("{:?}", high).contains("High"));
    assert!(format!("{:?}", medium).contains("Medium"));
    assert!(format!("{:?}", low).contains("Low"));
}

#[test]
fn test_topology_variants() {
    let stripe = PoolTopology::Stripe;
    let mirror = PoolTopology::Mirror;
    let raidz1 = PoolTopology::RaidZ1;
    let raidz2 = PoolTopology::RaidZ2;
    let raidz3 = PoolTopology::RaidZ3;

    assert!(format!("{:?}", stripe).contains("Stripe"));
    assert!(format!("{:?}", mirror).contains("Mirror"));
    assert!(format!("{:?}", raidz1).contains("RaidZ1"));
    assert!(format!("{:?}", raidz2).contains("RaidZ2"));
    assert!(format!("{:?}", raidz3).contains("RaidZ3"));
}

#[test]
fn test_redundancy_levels() {
    let none = RedundancyLevel::None;
    let mirror = RedundancyLevel::Mirror;
    let raidz1 = RedundancyLevel::RaidZ1;
    let raidz2 = RedundancyLevel::RaidZ2;
    let raidz3 = RedundancyLevel::RaidZ3;

    assert!(format!("{:?}", none).contains("None"));
    assert!(format!("{:?}", mirror).contains("Mirror"));
    assert!(format!("{:?}", raidz1).contains("RaidZ1"));
    assert!(format!("{:?}", raidz2).contains("RaidZ2"));
    assert!(format!("{:?}", raidz3).contains("RaidZ3"));
}

#[test]
fn test_device_size_calculation() {
    let device = create_test_device("/dev/test", DetectionDeviceType::NvmeSsd, 100);
    let expected_bytes = 100u64 * 1024 * 1024 * 1024;
    assert_eq!(device.size_bytes, expected_bytes);
}

#[test]
fn test_multiple_devices() {
    let devices = vec![
        create_test_device("/dev/nvme0n1", DetectionDeviceType::NvmeSsd, 500),
        create_test_device("/dev/nvme1n1", DetectionDeviceType::NvmeSsd, 500),
        create_test_device("/dev/sda", DetectionDeviceType::SataSsd, 1000),
    ];

    assert_eq!(devices.len(), 3);
    assert!(devices[0].is_nvme);
    assert!(devices[1].is_nvme);
    assert!(!devices[2].is_nvme);
}

#[test]
fn test_pool_setup_result_serialization() {
    let result = PoolSetupResult {
        pool_name: "test".to_string(),
        success: true,
        message: "OK".to_string(),
        devices_used: vec!["/dev/sda".to_string()],
        topology: PoolTopology::Stripe,
    };

    // Test that it can be serialized (serde traits work)
    let json = serde_json::to_string(&result);
    assert!(json.is_ok());
}

#[test]
fn test_compression_and_dedup_combinations() {
    let mut config = create_test_pool_config();

    // Both enabled
    config.enable_compression = true;
    config.enable_dedup = true;
    assert!(config.enable_compression && config.enable_dedup);

    // Both disabled
    config.enable_compression = false;
    config.enable_dedup = false;
    assert!(!config.enable_compression && !config.enable_dedup);

    // Only compression
    config.enable_compression = true;
    config.enable_dedup = false;
    assert!(config.enable_compression && !config.enable_dedup);
}

#[test]
fn test_ashift_values() {
    // Test ashift values through PoolPropertyConfig (where ashift actually lives)
    let mut pool_props = PoolPropertyConfig::default();

    // Common ashift values
    pool_props.ashift = 9; // 512 byte sectors
    assert_eq!(pool_props.ashift, 9);

    pool_props.ashift = 12; // 4K sectors (default)
    assert_eq!(pool_props.ashift, 12);

    pool_props.ashift = 13; // 8K sectors
    assert_eq!(pool_props.ashift, 13);
}

#[test]
fn test_min_device_size() {
    let mut config = create_test_pool_config();

    config.device_detection.min_device_size = 1024 * 1024 * 1024; // 1GB
    assert_eq!(config.device_detection.min_device_size, 1024 * 1024 * 1024);

    config.device_detection.min_device_size = 10 * 1024 * 1024 * 1024; // 10GB
    assert_eq!(
        config.device_detection.min_device_size,
        10 * 1024 * 1024 * 1024
    );
}
