// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Tests for pool setup types, errors, and results
//!
//! This module tests serialization, error handling, and type conversions
//! for pool setup operations.

#[cfg(test)]
#[expect(
    clippy::float_cmp,
    reason = "pool setup type tests compare exact literals and serde round-trip values"
)]
mod types_tests {
    use super::super::*;

    // ==================== PoolSetupError Tests ====================

    #[test]
    fn test_pool_setup_error_device_validation() {
        let error = PoolSetupError::DeviceValidation("invalid device".to_string());
        let display = format!("{error}");

        assert!(display.contains("Device validation failed"));
        assert!(display.contains("invalid device"));
    }

    #[test]
    fn test_pool_setup_error_pool_creation() {
        let error = PoolSetupError::PoolCreation("creation failed".to_string());
        let display = format!("{error}");

        assert!(display.contains("Pool creation failed"));
        assert!(display.contains("creation failed"));
    }

    #[test]
    fn test_pool_setup_error_configuration() {
        let error = PoolSetupError::Configuration("bad config".to_string());
        let display = format!("{error}");

        assert!(display.contains("Configuration error"));
        assert!(display.contains("bad config"));
    }

    #[test]
    fn test_pool_setup_error_device_scanning() {
        let error = PoolSetupError::DeviceScanning("scan failed".to_string());
        let display = format!("{error}");

        assert!(display.contains("Device scanning failed"));
        assert!(display.contains("scan failed"));
    }

    #[test]
    fn test_pool_setup_error_insufficient_devices() {
        let error = PoolSetupError::InsufficientDevices("need 3, got 2".to_string());
        let display = format!("{error}");

        assert!(display.contains("Insufficient devices"));
        assert!(display.contains("need 3, got 2"));
    }

    #[test]
    fn test_pool_setup_error_zfs_command() {
        let error = PoolSetupError::ZfsCommand("zpool create failed".to_string());
        let display = format!("{error}");

        assert!(display.contains("ZFS command failed"));
        assert!(display.contains("zpool create failed"));
    }

    #[test]
    fn test_pool_setup_error_debug() {
        let error = PoolSetupError::DeviceValidation("test".to_string());
        let debug = format!("{error:?}");

        assert!(debug.contains("DeviceValidation"));
    }

    #[test]
    fn test_pool_setup_error_from_core_error() {
        let core_error = nestgate_core::NestGateError::storage_error("test");
        let pool_error: PoolSetupError = core_error.into();

        let display = format!("{pool_error}");
        assert!(display.contains("Core error"));
    }

    // ==================== PoolSetupResult Tests ====================

    #[test]
    fn test_pool_setup_result_creation_success() {
        let result = PoolSetupResult {
            pool_name: "testpool".to_string(),
            success: true,
            message: "Pool created successfully".to_string(),
            devices_used: vec!["/dev/sda".to_string(), "/dev/sdb".to_string()],
            topology: PoolTopology::Mirror,
        };

        assert!(result.success);
        assert_eq!(result.pool_name, "testpool");
        assert_eq!(result.devices_used.len(), 2);
    }

    #[test]
    fn test_pool_setup_result_creation_failure() {
        let result = PoolSetupResult {
            pool_name: "testpool".to_string(),
            success: false,
            message: "Insufficient devices".to_string(),
            devices_used: vec![],
            topology: PoolTopology::Single,
        };

        assert!(!result.success);
        assert!(result.devices_used.is_empty());
        assert!(result.message.contains("Insufficient"));
    }

    #[test]
    fn test_pool_setup_result_serialization() {
        let result = PoolSetupResult {
            pool_name: "testpool".to_string(),
            success: true,
            message: "Created".to_string(),
            devices_used: vec!["/dev/sda".to_string()],
            topology: PoolTopology::Single,
        };

        let json = serde_json::to_string(&result).expect("Failed to serialize");
        assert!(json.contains("testpool"));
        assert!(json.contains("true"));
        assert!(json.contains("/dev/sda"));
    }

    #[test]
    fn test_pool_setup_result_deserialization() {
        let json = r#"{
            "pool_name": "mypool",
            "success": true,
            "message": "Pool created",
            "devices_used": ["/dev/nvme0n1"],
            "topology": "Single"
        }"#;

        let result: PoolSetupResult = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(result.pool_name, "mypool");
        assert!(result.success);
        assert_eq!(result.devices_used.len(), 1);
    }

    #[test]
    fn test_pool_setup_result_clone() {
        let result1 = PoolSetupResult {
            pool_name: "testpool".to_string(),
            success: true,
            message: "Created".to_string(),
            devices_used: vec!["/dev/sda".to_string()],
            topology: PoolTopology::Single,
        };

        let result2 = result1.clone();

        assert_eq!(result1.pool_name, result2.pool_name);
        assert_eq!(result1.success, result2.success);
        assert_eq!(result1.devices_used.len(), result2.devices_used.len());
    }

    #[test]
    fn test_pool_setup_result_debug() {
        let result = PoolSetupResult {
            pool_name: "testpool".to_string(),
            success: true,
            message: "Created".to_string(),
            devices_used: vec![],
            topology: PoolTopology::Single,
        };

        let debug = format!("{result:?}");
        assert!(debug.contains("PoolSetupResult"));
        assert!(debug.contains("testpool"));
    }

    #[test]
    fn test_pool_setup_result_with_all_topologies() {
        let topologies = vec![
            PoolTopology::Single,
            PoolTopology::Mirror,
            PoolTopology::RaidZ1,
            PoolTopology::RaidZ2,
            PoolTopology::RaidZ3,
        ];

        for topology in topologies {
            let result = PoolSetupResult {
                pool_name: "test".to_string(),
                success: true,
                message: "Created".to_string(),
                devices_used: vec![],
                topology: topology.clone(),
            };

            // Should be able to create result with any topology
            assert!(result.success);
        }
    }

    #[test]
    fn test_pool_setup_result_with_many_devices() {
        let mut devices = Vec::new();
        for i in 0..20 {
            devices.push(format!("/dev/sd{}", (b'a' + i) as char));
        }

        let result = PoolSetupResult {
            pool_name: "bigpool".to_string(),
            success: true,
            message: "Created with 20 devices".to_string(),
            devices_used: devices.clone(),
            topology: PoolTopology::RaidZ2,
        };

        assert_eq!(result.devices_used.len(), 20);
    }

    #[test]
    fn test_pool_setup_result_with_empty_message() {
        let result = PoolSetupResult {
            pool_name: "testpool".to_string(),
            success: true,
            message: String::new(),
            devices_used: vec![],
            topology: PoolTopology::Single,
        };

        assert!(result.message.is_empty());
    }

    #[test]
    fn test_pool_setup_result_with_long_message() {
        let long_message = "x".repeat(1000);
        let result = PoolSetupResult {
            pool_name: "testpool".to_string(),
            success: false,
            message: long_message.clone(),
            devices_used: vec![],
            topology: PoolTopology::Single,
        };

        assert_eq!(result.message.len(), 1000);
    }

    #[test]
    fn test_pool_setup_result_with_special_chars() {
        let result = PoolSetupResult {
            pool_name: "test-pool_123".to_string(),
            success: true,
            message: "Pool created with 测试 data 🚀".to_string(),
            devices_used: vec!["/dev/nvme0n1p1".to_string()],
            topology: PoolTopology::Single,
        };

        assert!(result.message.contains("测试"));
        assert!(result.message.contains("🚀"));
    }

    // ==================== PoolTopology Tests ====================

    #[test]
    fn test_pool_topology_serialization() {
        let topologies = vec![
            ("Single", PoolTopology::Single),
            ("Mirror", PoolTopology::Mirror),
            ("RaidZ1", PoolTopology::RaidZ1),
            ("RaidZ2", PoolTopology::RaidZ2),
            ("RaidZ3", PoolTopology::RaidZ3),
        ];

        for (expected_str, topology) in topologies {
            let json = serde_json::to_string(&topology).expect("Failed to serialize");
            assert!(json.contains(expected_str));
        }
    }

    #[test]
    fn test_pool_topology_clone() {
        let top1 = PoolTopology::RaidZ2;
        let top2 = top1.clone();

        // Both should serialize to the same value
        let json1 = serde_json::to_string(&top1).unwrap();
        let json2 = serde_json::to_string(&top2).unwrap();
        assert_eq!(json1, json2);
    }

    #[test]
    fn test_pool_topology_debug() {
        let topology = PoolTopology::Mirror;
        let debug = format!("{topology:?}");

        assert!(debug.contains("Mirror"));
    }

    // ==================== Device Type Conversion Tests ====================

    #[test]
    fn test_convert_all_device_types() {
        let conversions = vec![
            (DetectionDeviceType::NvmeSsd, ConfigDeviceType::NvmeSsd),
            (DetectionDeviceType::SataSsd, ConfigDeviceType::SataSsd),
            (DetectionDeviceType::Hdd, ConfigDeviceType::SpinningDisk),
            (
                DetectionDeviceType::OptaneMemory,
                ConfigDeviceType::OptaneMemory,
            ),
            (DetectionDeviceType::Unknown, ConfigDeviceType::SpinningDisk),
        ];

        for (detection, expected_config) in conversions {
            let config = convert_device_type(&detection);

            // Compare using debug representation since we can't directly compare
            let config_debug = format!("{config:?}");
            let expected_debug = format!("{expected_config:?}");
            assert_eq!(config_debug, expected_debug);
        }
    }

    // ==================== Integration Tests ====================

    #[test]
    fn test_full_result_lifecycle() {
        // Create result
        let result = PoolSetupResult {
            pool_name: "production_pool".to_string(),
            success: true,
            message: "Pool created successfully with mirror topology".to_string(),
            devices_used: vec!["/dev/nvme0n1".to_string(), "/dev/nvme1n1".to_string()],
            topology: PoolTopology::Mirror,
        };

        // Serialize
        let json = serde_json::to_string(&result).expect("Serialization failed");

        // Deserialize
        let restored: PoolSetupResult =
            serde_json::from_str(&json).expect("Deserialization failed");

        // Verify
        assert_eq!(result.pool_name, restored.pool_name);
        assert_eq!(result.success, restored.success);
        assert_eq!(result.devices_used.len(), restored.devices_used.len());
    }

    #[test]
    fn test_error_chain() {
        let core_error = nestgate_core::NestGateError::storage_error("test");
        let pool_error: PoolSetupError = core_error.into();

        // Should be able to display the error chain
        let display = format!("{pool_error}");
        assert!(!display.is_empty());
    }
}
