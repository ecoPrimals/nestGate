// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Comprehensive tests for device detection types and enums
//!
//! Tests device types, speed classes, and storage device structures.

use super::device_detection::*;
use super::*;

// ==================== DeviceType Tests ====================

#[test]
fn test_device_type_nvme_ssd() {
    let dtype = DetectionDeviceType::NvmeSsd;
    let json = serde_json::to_string(&dtype).expect("Failed to serialize");

    assert!(json.contains("NvmeSsd"));
}

#[test]
fn test_device_type_sata_ssd() {
    let dtype = DetectionDeviceType::SataSsd;
    let json = serde_json::to_string(&dtype).expect("Failed to serialize");

    assert!(json.contains("SataSsd"));
}

#[test]
fn test_device_type_hdd() {
    let dtype = DetectionDeviceType::Hdd;
    let json = serde_json::to_string(&dtype).expect("Failed to serialize");

    assert!(json.contains("Hdd"));
}

#[test]
fn test_device_type_optane() {
    let dtype = DetectionDeviceType::OptaneMemory;
    let json = serde_json::to_string(&dtype).expect("Failed to serialize");

    assert!(json.contains("OptaneMemory"));
}

#[test]
fn test_device_type_unknown() {
    let dtype = DetectionDeviceType::Unknown;
    let json = serde_json::to_string(&dtype).expect("Failed to serialize");

    assert!(json.contains("Unknown"));
}

#[test]
fn test_device_type_equality() {
    let dtype1 = DetectionDeviceType::NvmeSsd;
    let dtype2 = DetectionDeviceType::NvmeSsd;
    let dtype3 = DetectionDeviceType::SataSsd;

    assert_eq!(dtype1, dtype2);
    assert_ne!(dtype1, dtype3);
}

#[test]
fn test_device_type_clone() {
    let dtype1 = DetectionDeviceType::OptaneMemory;
    let dtype2 = dtype1;

    assert_eq!(dtype1, dtype2);
}

#[test]
fn test_device_type_debug() {
    let dtype = DetectionDeviceType::NvmeSsd;
    let debug = format!("{dtype:?}");

    assert!(debug.contains("NvmeSsd"));
}

#[test]
fn test_device_type_hash() {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert(DetectionDeviceType::NvmeSsd, "fast");
    map.insert(DetectionDeviceType::Hdd, "slow");

    assert_eq!(map.len(), 2);
    assert_eq!(map.get(&DetectionDeviceType::NvmeSsd), Some(&"fast"));
}

#[test]
fn test_device_type_all_variants() {
    let types = vec![
        DetectionDeviceType::NvmeSsd,
        DetectionDeviceType::SataSsd,
        DetectionDeviceType::Hdd,
        DetectionDeviceType::OptaneMemory,
        DetectionDeviceType::Unknown,
    ];

    for dtype in types {
        let json = serde_json::to_string(&dtype).expect("Failed to serialize");
        assert!(!json.is_empty());
    }
}

// ==================== SpeedClass Tests ====================

#[test]
fn test_speed_class_ultra_fast() {
    let speed = SpeedClass::UltraFast;
    let json = serde_json::to_string(&speed).expect("Failed to serialize");

    assert!(json.contains("UltraFast"));
}

#[test]
fn test_speed_class_fast() {
    let speed = SpeedClass::Fast;
    let json = serde_json::to_string(&speed).expect("Failed to serialize");

    assert!(json.contains("Fast"));
}

#[test]
fn test_speed_class_medium() {
    let speed = SpeedClass::Medium;
    let json = serde_json::to_string(&speed).expect("Failed to serialize");

    assert!(json.contains("Medium"));
}

#[test]
fn test_speed_class_slow() {
    let speed = SpeedClass::Slow;
    let json = serde_json::to_string(&speed).expect("Failed to serialize");

    assert!(json.contains("Slow"));
}

#[test]
fn test_speed_class_ordering() {
    // SpeedClass derives Ord, so ordering follows declaration order:
    // UltraFast < Fast < Medium < Slow
    assert!(SpeedClass::UltraFast < SpeedClass::Fast);
    assert!(SpeedClass::Fast < SpeedClass::Medium);
    assert!(SpeedClass::Medium < SpeedClass::Slow);
}

#[test]
fn test_speed_class_equality() {
    let speed1 = SpeedClass::Fast;
    let speed2 = SpeedClass::Fast;
    let speed3 = SpeedClass::Slow;

    assert_eq!(speed1, speed2);
    assert_ne!(speed1, speed3);
}

#[test]
fn test_speed_class_clone() {
    let speed1 = SpeedClass::UltraFast;
    let speed2 = speed1;

    assert_eq!(speed1, speed2);
}

#[test]
fn test_speed_class_debug() {
    let speed = SpeedClass::Medium;
    let debug = format!("{speed:?}");

    assert!(debug.contains("Medium"));
}

#[test]
fn test_speed_class_hash() {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert(SpeedClass::UltraFast, 1000);
    map.insert(SpeedClass::Fast, 500);
    map.insert(SpeedClass::Medium, 200);
    map.insert(SpeedClass::Slow, 100);

    assert_eq!(map.len(), 4);
    assert_eq!(map.get(&SpeedClass::UltraFast), Some(&1000));
}

#[test]
fn test_speed_class_sorting() {
    let mut speeds = [
        SpeedClass::Fast,
        SpeedClass::Slow,
        SpeedClass::UltraFast,
        SpeedClass::Medium,
    ];

    speeds.sort();

    // After sorting: UltraFast < Fast < Medium < Slow (declaration order)
    assert_eq!(speeds[0], SpeedClass::UltraFast);
    assert_eq!(speeds[1], SpeedClass::Fast);
    assert_eq!(speeds[2], SpeedClass::Medium);
    assert_eq!(speeds[3], SpeedClass::Slow);
}

// ==================== StorageDevice Tests ====================

/// Creates  Test Storage Device
fn create_test_storage_device() -> StorageDevice {
    StorageDevice {
        device_path: "/dev/nvme0n1".to_string(),
        model: "Samsung 980 PRO".to_string(),
        size_bytes: 1_000_000_000_000, // 1TB
        device_type: DetectionDeviceType::NvmeSsd,
        speed_class: SpeedClass::Fast,
        in_use: false,
        current_use: None,
    }
}

#[test]
fn test_storage_device_creation() {
    let device = create_test_storage_device();

    assert_eq!(device.device_path, "/dev/nvme0n1");
    assert_eq!(device.size_bytes, 1_000_000_000_000);
    assert!(!device.in_use);
}

#[test]
fn test_storage_device_clone() {
    let device1 = create_test_storage_device();
    let device2 = device1.clone();

    assert_eq!(device1.device_path, device2.device_path);
    assert_eq!(device1.size_bytes, device2.size_bytes);
}

#[test]
fn test_storage_device_debug() {
    let device = create_test_storage_device();
    let debug = format!("{device:?}");

    assert!(debug.contains("StorageDevice"));
    assert!(debug.contains("/dev/nvme0n1"));
}

#[test]
fn test_storage_device_serialization() {
    let device = create_test_storage_device();
    let json = serde_json::to_string(&device).expect("Failed to serialize");

    assert!(json.contains("/dev/nvme0n1"));
    assert!(json.contains("Samsung 980 PRO"));
}

#[test]
fn test_storage_device_deserialization() {
    let device = create_test_storage_device();
    let json = serde_json::to_string(&device).expect("Failed to serialize");
    let restored: StorageDevice = serde_json::from_str(&json).expect("Failed to deserialize");

    assert_eq!(device.device_path, restored.device_path);
    assert_eq!(device.size_bytes, restored.size_bytes);
}

#[test]
fn test_storage_device_in_use() {
    let mut device = create_test_storage_device();
    device.in_use = true;
    device.current_use = Some("ext4 filesystem".to_string());

    assert!(device.in_use);
    assert!(device.current_use.is_some());
    assert_eq!(device.current_use.unwrap(), "ext4 filesystem");
}

#[test]
fn test_storage_device_not_in_use() {
    let device = create_test_storage_device();

    assert!(!device.in_use);
    assert!(device.current_use.is_none());
}

#[test]
fn test_storage_device_different_types() {
    let types = vec![
        DetectionDeviceType::NvmeSsd,
        DetectionDeviceType::SataSsd,
        DetectionDeviceType::Hdd,
        DetectionDeviceType::OptaneMemory,
        DetectionDeviceType::Unknown,
    ];

    for dtype in types {
        let device = StorageDevice {
            device_path: "/dev/sda".to_string(),
            model: "Test".to_string(),
            size_bytes: 1000,
            device_type: dtype,
            speed_class: SpeedClass::Fast,
            in_use: false,
            current_use: None,
        };

        assert_eq!(device.device_type, dtype);
    }
}

#[test]
fn test_storage_device_different_speeds() {
    let speeds = vec![
        SpeedClass::UltraFast,
        SpeedClass::Fast,
        SpeedClass::Medium,
        SpeedClass::Slow,
    ];

    for speed in speeds {
        let device = StorageDevice {
            device_path: "/dev/sda".to_string(),
            model: "Test".to_string(),
            size_bytes: 1000,
            device_type: DetectionDeviceType::NvmeSsd,
            speed_class: speed,
            in_use: false,
            current_use: None,
        };

        assert_eq!(device.speed_class, speed);
    }
}

#[test]
fn test_storage_device_various_sizes() {
    let sizes = vec![
        1_000_000,          // 1MB
        1_000_000_000,      // 1GB
        1_000_000_000_000,  // 1TB
        10_000_000_000_000, // 10TB
        u64::MAX,           // Maximum
    ];

    for size in sizes {
        let device = StorageDevice {
            device_path: "/dev/sda".to_string(),
            model: "Test".to_string(),
            size_bytes: size,
            device_type: DetectionDeviceType::NvmeSsd,
            speed_class: SpeedClass::Fast,
            in_use: false,
            current_use: None,
        };

        assert_eq!(device.size_bytes, size);
    }
}

#[test]
fn test_storage_device_empty_model() {
    let device = StorageDevice {
        device_path: "/dev/sda".to_string(),
        model: String::new(),
        size_bytes: 1000,
        device_type: DetectionDeviceType::Unknown,
        speed_class: SpeedClass::Slow,
        in_use: false,
        current_use: None,
    };

    assert!(device.model.is_empty());
}

#[test]
fn test_storage_device_special_paths() {
    let paths = vec![
        "/dev/nvme0n1",
        "/dev/nvme0n1p1",
        "/dev/sda",
        "/dev/sdb1",
        "/dev/loop0",
        "/dev/md0",
    ];

    for path in paths {
        let device = StorageDevice {
            device_path: path.to_string(),
            model: "Test".to_string(),
            size_bytes: 1000,
            device_type: DetectionDeviceType::NvmeSsd,
            speed_class: SpeedClass::Fast,
            in_use: false,
            current_use: None,
        };

        assert_eq!(device.device_path, path);
    }
}

#[test]
fn test_storage_device_current_use_options() {
    let uses = vec![
        None,
        Some("ext4".to_string()),
        Some("xfs".to_string()),
        Some("zfs".to_string()),
        Some("swap".to_string()),
        Some("LVM".to_string()),
    ];

    for usage in uses {
        let device = StorageDevice {
            device_path: "/dev/sda".to_string(),
            model: "Test".to_string(),
            size_bytes: 1000,
            device_type: DetectionDeviceType::NvmeSsd,
            speed_class: SpeedClass::Fast,
            in_use: usage.is_some(),
            current_use: usage.clone(),
        };

        assert_eq!(device.current_use, usage);
    }
}

// ==================== DeviceScanner Tests ====================

#[test]
fn test_device_scanner_creation() {
    let config = DeviceDetectionConfig::default();
    let scanner = DeviceScanner::new(config);

    // Scanner should be created successfully
    assert!(std::mem::size_of_val(&scanner) > 0);
}

#[test]
fn test_device_scanner_with_custom_config() {
    let config = DeviceDetectionConfig {
        scan_paths: vec!["/dev/nvme*".to_string()],
        exclude_patterns: vec!["loop".to_string()],
        include_removable: false,
        min_device_size: 500_000_000,
        max_device_size: 0,
        skip_mountpoints: vec!["/boot".to_string()],
        skip_fstypes: vec!["vfat".to_string()],
        include_loop_devices: false,
    };

    let scanner = DeviceScanner::new(config);
    assert!(std::mem::size_of_val(&scanner) > 0);
}

// ==================== Edge Cases ====================

#[test]
fn test_device_type_round_trip_serialization() {
    let types = vec![
        DetectionDeviceType::NvmeSsd,
        DetectionDeviceType::SataSsd,
        DetectionDeviceType::Hdd,
        DetectionDeviceType::OptaneMemory,
        DetectionDeviceType::Unknown,
    ];

    for dtype in types {
        let json = serde_json::to_string(&dtype).expect("Serialization failed");
        let restored: DetectionDeviceType =
            serde_json::from_str(&json).expect("Deserialization failed");

        let json2 = serde_json::to_string(&restored).unwrap();
        assert_eq!(json, json2);
    }
}

#[test]
fn test_speed_class_round_trip_serialization() {
    let speeds = vec![
        SpeedClass::UltraFast,
        SpeedClass::Fast,
        SpeedClass::Medium,
        SpeedClass::Slow,
    ];

    for speed in speeds {
        let json = serde_json::to_string(&speed).expect("Serialization failed");
        let restored: SpeedClass = serde_json::from_str(&json).expect("Deserialization failed");

        let json2 = serde_json::to_string(&restored).unwrap();
        assert_eq!(json, json2);
    }
}

#[test]
fn test_storage_device_with_unicode() {
    let device = StorageDevice {
        device_path: "/dev/sda".to_string(),
        model: "Samsung 测试 🚀".to_string(),
        size_bytes: 1000,
        device_type: DetectionDeviceType::NvmeSsd,
        speed_class: SpeedClass::Fast,
        in_use: false,
        current_use: Some("测试文件系统".to_string()),
    };

    let json = serde_json::to_string(&device).expect("Failed to serialize");
    let restored: StorageDevice = serde_json::from_str(&json).expect("Failed to deserialize");

    assert_eq!(device.model, restored.model);
    assert_eq!(device.current_use, restored.current_use);
}

#[test]
fn test_storage_device_zero_size() {
    let device = StorageDevice {
        device_path: "/dev/sda".to_string(),
        model: "Test".to_string(),
        size_bytes: 0,
        device_type: DetectionDeviceType::Unknown,
        speed_class: SpeedClass::Slow,
        in_use: false,
        current_use: None,
    };

    assert_eq!(device.size_bytes, 0);
}

#[test]
fn test_storage_device_max_size() {
    let device = StorageDevice {
        device_path: "/dev/sda".to_string(),
        model: "Test".to_string(),
        size_bytes: u64::MAX,
        device_type: DetectionDeviceType::NvmeSsd,
        speed_class: SpeedClass::UltraFast,
        in_use: false,
        current_use: None,
    };

    assert_eq!(device.size_bytes, u64::MAX);
}

#[test]
fn test_device_type_in_hashset() {
    use std::collections::HashSet;

    let mut set = HashSet::new();
    set.insert(DetectionDeviceType::NvmeSsd);
    set.insert(DetectionDeviceType::SataSsd);
    set.insert(DetectionDeviceType::NvmeSsd); // Duplicate

    assert_eq!(set.len(), 2);
    assert!(set.contains(&DetectionDeviceType::NvmeSsd));
}

#[test]
fn test_speed_class_in_btreemap() {
    use std::collections::BTreeMap;

    let mut map = BTreeMap::new();
    map.insert(SpeedClass::Fast, "NVMe");
    map.insert(SpeedClass::Slow, "HDD");
    map.insert(SpeedClass::UltraFast, "Optane");
    map.insert(SpeedClass::Medium, "SATA SSD");

    // BTreeMap maintains sorted order: UltraFast < Fast < Medium < Slow (declaration order)
    let keys: Vec<_> = map.keys().collect();
    assert_eq!(keys.len(), 4);
    assert_eq!(keys[0], &SpeedClass::UltraFast);
    assert_eq!(keys[1], &SpeedClass::Fast);
    assert_eq!(keys[2], &SpeedClass::Medium);
    assert_eq!(keys[3], &SpeedClass::Slow);
}
