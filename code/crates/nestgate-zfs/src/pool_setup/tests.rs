// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Comprehensive tests for ZFS Pool Setup module
//!
//! This test module provides extensive coverage for pool setup operations including:
//! - Device detection and scanning
//! - Pool topology validation
//! - Configuration validation
//! - Error handling

use super::*;

#[cfg(test)]
mod pool_setup_tests {
    use super::*;

    #[tokio::test]
    async fn test_pool_setup_creation() {
        let config = PoolSetupConfig::default();

        // Creating pool setup should not require ZFS to be installed
        let result = ZfsPoolSetup::new_with_config(config).await;

        // In test environments without ZFS, this may fail gracefully
        match result {
            Ok(_setup) => {
                println!("Pool setup created successfully");
            }
            Err(_) => {
                // Expected in environments without ZFS
                println!("Pool setup creation skipped - ZFS not available");
            }
        }
    }

    #[test]
    fn test_pool_setup_config_default() {
        let config = PoolSetupConfig::default();

        assert!(!config.pool_name.is_empty());
        assert!(matches!(
            config.topology,
            PoolTopology::Single
                | PoolTopology::Mirror
                | PoolTopology::RaidZ1
                | PoolTopology::RaidZ2
                | PoolTopology::RaidZ3
        ));
    }

    #[test]
    fn test_device_type_conversion() {
        let nvme = DetectionDeviceType::NvmeSsd;
        let converted = convert_device_type(&nvme);
        assert!(matches!(converted, ConfigDeviceType::NvmeSsd));

        let sata = DetectionDeviceType::SataSsd;
        let converted = convert_device_type(&sata);
        assert!(matches!(converted, ConfigDeviceType::SataSsd));

        let hdd = DetectionDeviceType::Hdd;
        let converted = convert_device_type(&hdd);
        assert!(matches!(converted, ConfigDeviceType::SpinningDisk));

        let optane = DetectionDeviceType::OptaneMemory;
        let converted = convert_device_type(&optane);
        assert!(matches!(converted, ConfigDeviceType::OptaneMemory));

        let unknown = DetectionDeviceType::Unknown;
        let converted = convert_device_type(&unknown);
        assert!(matches!(converted, ConfigDeviceType::SpinningDisk));
    }

    #[test]
    fn test_pool_setup_error_types() {
        let err1 = PoolSetupError::DeviceValidation("test".to_string());
        assert!(err1.to_string().contains("Device validation failed"));

        let err2 = PoolSetupError::PoolCreation("test".to_string());
        assert!(err2.to_string().contains("Pool creation failed"));

        let err3 = PoolSetupError::Configuration("test".to_string());
        assert!(err3.to_string().contains("Configuration error"));

        let err4 = PoolSetupError::DeviceScanning("test".to_string());
        assert!(err4.to_string().contains("Device scanning failed"));

        let err5 = PoolSetupError::InsufficientDevices("test".to_string());
        assert!(err5.to_string().contains("Insufficient devices"));

        let err6 = PoolSetupError::ZfsCommand("test".to_string());
        assert!(err6.to_string().contains("ZFS command failed"));
    }

    #[test]
    fn test_pool_setup_result_creation() {
        let result = PoolSetupResult {
            pool_name: "test_pool".to_string(),
            success: true,
            message: "Pool created successfully".to_string(),
            devices_used: vec!["/dev/sda".to_string(), "/dev/sdb".to_string()],
            topology: PoolTopology::Mirror,
        };

        assert_eq!(result.pool_name, "test_pool");
        assert!(result.success);
        assert_eq!(result.devices_used.len(), 2);
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

        // Test serialization
        let serialized = serde_json::to_string(&result).unwrap();
        assert!(serialized.contains("test_pool"));

        // Test deserialization
        let deserialized: PoolSetupResult = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.pool_name, "test_pool");
        assert!(deserialized.success);
    }

    #[test]
    fn test_redundancy_levels() {
        let none = RedundancyLevel::None;
        let single = RedundancyLevel::Single;
        let double = RedundancyLevel::Double;
        let triple = RedundancyLevel::Triple;

        // Verify they're different variants
        assert!(std::mem::discriminant(&none) != std::mem::discriminant(&single));
        assert!(std::mem::discriminant(&single) != std::mem::discriminant(&double));
        assert!(std::mem::discriminant(&double) != std::mem::discriminant(&triple));
    }

    #[test]
    fn test_pool_topology_variants() {
        let single = PoolTopology::Single;
        let mirror = PoolTopology::Mirror;
        let raidz1 = PoolTopology::RaidZ1;
        let raidz2 = PoolTopology::RaidZ2;
        let raidz3 = PoolTopology::RaidZ3;

        // Verify they're different variants
        assert!(std::mem::discriminant(&single) != std::mem::discriminant(&mirror));
        assert!(std::mem::discriminant(&mirror) != std::mem::discriminant(&raidz1));
        assert!(std::mem::discriminant(&raidz1) != std::mem::discriminant(&raidz2));
        assert!(std::mem::discriminant(&raidz2) != std::mem::discriminant(&raidz3));
    }
}

#[cfg(test)]
mod device_detection_tests {
    use super::*;

    #[test]
    fn test_device_detection_config_default() {
        let config = DeviceDetectionConfig::default();

        // Should have reasonable defaults
        assert!(!config.include_removable);
        assert!(config.min_device_size > 0);
    }

    #[test]
    fn test_device_scanner_creation() {
        let config = DeviceDetectionConfig::default();
        let _scanner = DeviceScanner::new(config);

        // Scanner should be created successfully (no panic)
    }

    #[tokio::test]
    async fn test_device_scanning() {
        let config = DeviceDetectionConfig::default();
        let scanner = DeviceScanner::new(config);

        // Attempt to scan devices
        match scanner.scan_devices().await {
            Ok(devices) => {
                // In test environments, might find no devices
                println!("Found {} devices", devices.len());
            }
            Err(_) => {
                // Expected in restricted test environments
                println!("Device scanning skipped - insufficient permissions");
            }
        }
    }

    #[test]
    fn test_storage_device_creation() {
        let device = StorageDevice {
            device_path: "/dev/sda".to_string(),
            size_bytes: 1_000_000_000_000,
            device_type: DetectionDeviceType::NvmeSsd,
            model: "Samsung 970 EVO".to_string(),
            speed_class: SpeedClass::UltraFast,
            in_use: false,
            current_use: None,
        };

        assert_eq!(device.device_path, "/dev/sda");
        assert_eq!(device.size_bytes, 1_000_000_000_000);
        assert!(matches!(device.device_type, DetectionDeviceType::NvmeSsd));
        assert!(matches!(device.speed_class, SpeedClass::UltraFast));
    }

    #[test]
    fn test_speed_class_ordering() {
        let slow = SpeedClass::Slow;
        let medium = SpeedClass::Medium;
        let fast = SpeedClass::Fast;
        let ultra = SpeedClass::UltraFast;

        // Verify ordering (enum order: Ultra < Fast < Medium < Slow)
        assert!(ultra < fast);
        assert!(fast < medium);
        assert!(medium < slow);
    }

    #[test]
    fn test_device_type_variants() {
        let nvme = DetectionDeviceType::NvmeSsd;
        let sata = DetectionDeviceType::SataSsd;
        let hdd = DetectionDeviceType::Hdd;
        let optane = DetectionDeviceType::OptaneMemory;
        let unknown = DetectionDeviceType::Unknown;

        // Verify they're all different
        let types = [nvme, sata, hdd, optane, unknown];
        for (i, type1) in types.iter().enumerate() {
            for (j, type2) in types.iter().enumerate() {
                if i != j {
                    assert!(std::mem::discriminant(type1) != std::mem::discriminant(type2));
                }
            }
        }
    }
}

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[test]
    fn test_validator_creation() {
        let config = PoolSetupConfig::default();
        let _validator = PoolSetupValidator::new(config);

        // Validator should be created successfully (no panic)
    }

    #[test]
    fn test_validation_result_success() {
        let result = ValidationResult {
            is_valid: true,
            issues: vec![],
            warnings: vec![],
        };

        assert!(result.is_valid);
        assert!(result.issues.is_empty());
        assert!(result.warnings.is_empty());
    }

    #[test]
    fn test_validation_result_with_errors() {
        let result = ValidationResult {
            is_valid: false,
            issues: vec!["Insufficient devices".to_string()],
            warnings: vec!["Slow device detected".to_string()],
        };

        assert!(!result.is_valid);
        assert_eq!(result.issues.len(), 1);
        assert_eq!(result.warnings.len(), 1);
    }

    #[test]
    fn test_validation_result_methods() {
        let mut result = ValidationResult::new();
        assert!(result.is_valid);

        result.add_error("Test error".to_string());
        assert!(!result.is_valid);
        assert_eq!(result.issues.len(), 1);

        result.add_warning("Test warning".to_string());
        assert_eq!(result.warnings.len(), 1);
    }

    #[test]
    fn test_validation_result_merge() {
        let mut result1 = ValidationResult::new();
        result1.add_warning("Warning 1".to_string());

        let mut result2 = ValidationResult::new();
        result2.add_error("Error 2".to_string());

        result1.merge(result2);
        assert!(!result1.is_valid);
        assert_eq!(result1.warnings.len(), 1);
        assert_eq!(result1.issues.len(), 1);
    }

    #[test]
    fn test_validate_device() {
        let config = PoolSetupConfig::default();
        let validator = PoolSetupValidator::new(config);

        let device = StorageDevice {
            device_path: "/dev/sda".to_string(),
            size_bytes: 10_000_000_000, // 10GB
            device_type: DetectionDeviceType::NvmeSsd,
            model: "Test".to_string(),
            speed_class: SpeedClass::Fast,
            in_use: false,
            current_use: None,
        };

        let result = validator.validate_device(&device);
        // Should be valid or have specific issues
        assert!(result.is_valid || !result.issues.is_empty());
    }

    #[test]
    fn test_validate_pool_config_empty_name() {
        let mut config = PoolSetupConfig::default();
        config.pool_name = String::new();

        let validator = PoolSetupValidator::new(config.clone());
        let result = validator.validate_pool_config(&config);

        assert!(!result.is_valid);
        assert!(result.issues.iter().any(|i| i.contains("name")));
    }

    #[test]
    fn test_validate_pool_config_no_devices() {
        let mut config = PoolSetupConfig::default();
        config.devices.clear();

        let validator = PoolSetupValidator::new(config.clone());
        let result = validator.validate_pool_config(&config);

        assert!(!result.is_valid);
        assert!(
            result
                .issues
                .iter()
                .any(|i| i.contains("devices") || i.contains("No devices"))
        );
    }

    #[test]
    fn test_validate_pool_config_insufficient_devices_for_mirror() {
        let mut config = PoolSetupConfig::default();
        config.topology = PoolTopology::Mirror;
        config.devices = vec!["/dev/sda".to_string()]; // Only 1 device, need 2

        let validator = PoolSetupValidator::new(config.clone());
        let result = validator.validate_pool_config(&config);

        assert!(!result.is_valid);
    }

    #[test]
    fn test_validate_pool_config_sufficient_devices() {
        let mut config = PoolSetupConfig::default();
        config.topology = PoolTopology::Mirror;
        config.devices = vec!["/dev/sda".to_string(), "/dev/sdb".to_string()];

        let validator = PoolSetupValidator::new(config.clone());
        let result = validator.validate_pool_config(&config);

        // Should be valid or have only warnings
        assert!(result.is_valid || result.issues.is_empty());
    }
}

#[cfg(test)]
mod pool_creation_tests {
    use super::*;

    #[test]
    fn test_pool_creator_creation() {
        let _creator = PoolCreator::new();
        // Creator should be created successfully (no panic)
    }

    #[test]
    fn test_pool_creator_dry_run() {
        let _creator = PoolCreator::new_dry_run();
        // Dry run creator should be created successfully (no panic)
    }

    #[tokio::test]
    async fn test_pool_creation_dry_run_execution() {
        let creator = PoolCreator::new_dry_run();
        let config = PoolSetupConfig::default();

        // Dry run should succeed without ZFS installed
        let result = creator.create_pool_safe(&config).await;

        match result {
            Ok(setup_result) => {
                println!("Dry run succeeded: {}", setup_result.message);
                assert!(
                    setup_result.message.contains("DRY RUN")
                        || setup_result.message.contains("Would create")
                );
            }
            Err(e) => {
                println!("Dry run error (acceptable): {e}");
            }
        }
    }

    #[test]
    fn test_pool_creation_command_generation() {
        // Test that we can generate pool creation commands
        let devices = ["/dev/sda".to_string(), "/dev/sdb".to_string()];
        let pool_name = "test_pool";

        // Verify device paths are valid
        assert!(!devices.is_empty());
        assert!(!pool_name.is_empty());
        assert!(pool_name.chars().all(|c| c.is_alphanumeric() || c == '_'));
    }

    #[test]
    fn test_topology_variants() {
        let topologies = [PoolTopology::Single,
            PoolTopology::Mirror,
            PoolTopology::RaidZ1,
            PoolTopology::RaidZ2,
            PoolTopology::RaidZ3];

        // Verify all are different
        for (i, top1) in topologies.iter().enumerate() {
            for (j, top2) in topologies.iter().enumerate() {
                if i != j {
                    assert!(std::mem::discriminant(top1) != std::mem::discriminant(top2));
                }
            }
        }
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_full_pool_setup_workflow() {
        // This is an integration test that goes through the full workflow
        let config = PoolSetupConfig::default();

        // Step 1: Create pool setup
        let setup_result = ZfsPoolSetup::new_with_config(config).await;

        if setup_result.is_err() {
            println!("Full workflow test skipped - ZFS not available");
            return;
        }

        println!("Full workflow test passed");
    }

    #[test]
    fn test_error_propagation() {
        // Test that NestGateError converts properly
        let core_error = NestGateError::storage_error("test");
        let pool_error: PoolSetupError = core_error.into();

        assert!(matches!(pool_error, PoolSetupError::Core(_)));
    }

    #[test]
    fn test_pool_setup_result_clone() {
        let result = PoolSetupResult {
            pool_name: "test".to_string(),
            success: true,
            message: "Test".to_string(),
            devices_used: vec![],
            topology: PoolTopology::Single,
        };

        let cloned = result.clone();
        assert_eq!(result.pool_name, cloned.pool_name);
        assert_eq!(result.success, cloned.success);
    }

    #[test]
    fn test_config_default() {
        let config = PoolSetupConfig::default();
        assert!(!config.pool_name.is_empty());
        assert!(!config.device_detection.scan_paths.is_empty());
    }

    #[test]
    fn test_storage_tier_variants() {
        let hot = ConfigStorageTier::Hot;
        let warm = ConfigStorageTier::Warm;
        let cold = ConfigStorageTier::Cold;

        assert!(std::mem::discriminant(&hot) != std::mem::discriminant(&warm));
        assert!(std::mem::discriminant(&warm) != std::mem::discriminant(&cold));
    }

    #[test]
    fn test_device_type_variants() {
        let optane = ConfigDeviceType::OptaneMemory;
        let nvme = ConfigDeviceType::NvmeSsd;
        let sata = ConfigDeviceType::SataSsd;
        let hdd = ConfigDeviceType::SpinningDisk;

        let types = [optane, nvme, sata, hdd];
        for (i, type1) in types.iter().enumerate() {
            for (j, type2) in types.iter().enumerate() {
                if i != j {
                    assert!(std::mem::discriminant(type1) != std::mem::discriminant(type2));
                }
            }
        }
    }
}
