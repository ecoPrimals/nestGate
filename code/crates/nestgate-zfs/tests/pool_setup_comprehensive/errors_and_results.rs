// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! `PoolSetupError` and `PoolSetupResult` variants, display strings, and serde.

use nestgate_zfs::pool_setup::{PoolSetupError, PoolSetupResult, PoolTopology};

// ==================== POOL SETUP ERROR TESTS ====================

#[test]
fn test_pool_setup_error_device_validation() {
    let err = PoolSetupError::DeviceValidation("Invalid device".to_string());
    assert!(err.to_string().contains("Device validation failed"));
}

#[test]
fn test_pool_setup_error_pool_creation() {
    let err = PoolSetupError::PoolCreation("Failed to create".to_string());
    assert!(err.to_string().contains("Pool creation failed"));
}

#[test]
fn test_pool_setup_error_configuration() {
    let err = PoolSetupError::Configuration("Bad config".to_string());
    assert!(err.to_string().contains("Configuration error"));
}

#[test]
fn test_pool_setup_error_device_scanning() {
    let err = PoolSetupError::DeviceScanning("Scan failed".to_string());
    assert!(err.to_string().contains("Device scanning failed"));
}

#[test]
fn test_pool_setup_error_insufficient_devices() {
    let err = PoolSetupError::InsufficientDevices("Need more".to_string());
    assert!(err.to_string().contains("Insufficient devices"));
}

#[test]
fn test_pool_setup_error_zfs_command() {
    let err = PoolSetupError::ZfsCommand("Command failed".to_string());
    assert!(err.to_string().contains("ZFS command failed"));
}

#[test]
fn test_pool_setup_error_debug() {
    let err = PoolSetupError::Configuration("test".to_string());
    let debug_str = format!("{:?}", err);
    assert!(debug_str.contains("Configuration"));
}

// ==================== POOL SETUP RESULT TESTS ====================

#[test]
fn test_pool_setup_result_success() {
    let result = PoolSetupResult {
        pool_name: "test_pool".to_string(),
        success: true,
        message: "Pool created".to_string(),
        devices_used: vec!["/dev/sda".to_string()],
        topology: PoolTopology::Single,
    };

    assert!(result.success);
    assert_eq!(result.pool_name, "test_pool");
}

#[test]
fn test_pool_setup_result_failure() {
    let result = PoolSetupResult {
        pool_name: "test_pool".to_string(),
        success: false,
        message: "Creation failed".to_string(),
        devices_used: Vec::new(),
        topology: PoolTopology::Single,
    };

    assert!(!result.success);
    assert!(result.devices_used.is_empty());
}

#[test]
fn test_pool_setup_result_with_multiple_devices() {
    let devices = vec![
        "/dev/sda".to_string(),
        "/dev/sdb".to_string(),
        "/dev/sdc".to_string(),
    ];

    let result = PoolSetupResult {
        pool_name: "raid_pool".to_string(),
        success: true,
        message: "RAID pool created".to_string(),
        devices_used: devices.clone(),
        topology: PoolTopology::RaidZ1,
    };

    assert_eq!(result.devices_used.len(), 3);
}

#[test]
fn test_pool_setup_result_clone() {
    let result1 = PoolSetupResult {
        pool_name: "test_pool".to_string(),
        success: true,
        message: "Success".to_string(),
        devices_used: vec!["/dev/sda".to_string()],
        topology: PoolTopology::Mirror,
    };

    let result2 = result1.clone();
    assert_eq!(result1.pool_name, result2.pool_name);
    assert_eq!(result1.success, result2.success);
}

#[test]
fn test_pool_setup_result_serialization() {
    let result = PoolSetupResult {
        pool_name: "test_pool".to_string(),
        success: true,
        message: "Success".to_string(),
        devices_used: vec!["/dev/sda".to_string()],
        topology: PoolTopology::Single,
    };

    let json = serde_json::to_string(&result).expect("Should serialize");
    assert!(json.contains("test_pool"));
    assert!(json.contains("success"));
}

#[test]
fn test_pool_setup_result_deserialization() {
    let json = r#"{
        "pool_name": "my_pool",
        "success": true,
        "message": "Created",
        "devices_used": ["/dev/sda"],
        "topology": "Mirror"
    }"#;

    let result: PoolSetupResult = serde_json::from_str(json).expect("Should deserialize");
    assert_eq!(result.pool_name, "my_pool");
    assert!(result.success);
}
