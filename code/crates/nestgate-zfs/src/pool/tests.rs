//! Tests for ZFS pool management

#[cfg(test)]
mod pool_manager_tests {
    // Import types ONLY from pool module to avoid conflicts with root types
    use crate::config::ZfsConfig;
    use crate::pool::{PoolCapacity, PoolHealth, PoolInfo, PoolState, ZfsPoolManager};
    use crate::pool_helpers::parse_size_with_units;
    use std::collections::HashMap;

    // ==================== Pool State and Health Tests ====================

    #[test]
    fn test_pool_state_variants() {
        let online = PoolState::Online;
        let offline = PoolState::Offline;
        let degraded = PoolState::Degraded;
        let faulted = PoolState::Faulted;
        let unknown = PoolState::Unknown;

        assert_eq!(online, PoolState::Online);
        assert_eq!(offline, PoolState::Offline);
        assert_eq!(degraded, PoolState::Degraded);
        assert_eq!(faulted, PoolState::Faulted);
        assert_eq!(unknown, PoolState::Unknown);
    }

    #[test]
    fn test_pool_health_variants() {
        let healthy = PoolHealth::Healthy;
        let warning = PoolHealth::Warning;
        let critical = PoolHealth::Critical;
        let unknown = PoolHealth::Unknown;

        assert_eq!(healthy, PoolHealth::Healthy);
        assert_eq!(warning, PoolHealth::Warning);
        assert_eq!(critical, PoolHealth::Critical);
        assert_eq!(unknown, PoolHealth::Unknown);
    }

    #[test]
    fn test_pool_capacity_calculation() {
        let capacity = PoolCapacity {
            total_bytes: 1000,
            used_bytes: 400,
            available_bytes: 600,
            utilization_percent: 40.0,
        };

        assert_eq!(capacity.total_bytes, 1000);
        assert_eq!(capacity.used_bytes, 400);
        assert_eq!(capacity.available_bytes, 600);
        assert_eq!(capacity.utilization_percent, 40.0);
    }

    #[test]
    fn test_pool_info_creation() {
        let pool = PoolInfo {
            name: "test_pool".to_string(),
            state: PoolState::Online,
            health: PoolHealth::Healthy,
            capacity: PoolCapacity {
                total_bytes: 1_000_000_000,
                used_bytes: 500_000_000,
                available_bytes: 500_000_000,
                utilization_percent: 50.0,
            },
            devices: vec!["/dev/sda".to_string(), "/dev/sdb".to_string()],
            properties: HashMap::new(),
        };

        assert_eq!(pool.name, "test_pool");
        assert_eq!(pool.state, PoolState::Online);
        assert_eq!(pool.health, PoolHealth::Healthy);
        assert_eq!(pool.devices.len(), 2);
    }

    // ==================== Pool Manager Creation Tests ====================

    #[test]
    fn test_pool_manager_creation_for_testing() {
        let manager = ZfsPoolManager::new_for_testing();
        // Verify manager is created successfully
        assert!(format!("{:?}", manager).contains("ZfsPoolManager"));
    }

    #[test]
    fn test_pool_manager_production_creation() {
        let config = ZfsConfig::default();
        let manager = ZfsPoolManager::new_production(config);
        // Verify manager is created successfully
        assert!(format!("{:?}", manager).contains("ZfsPoolManager"));
    }

    #[tokio::test]
    async fn test_pool_manager_new_with_config() {
        let config = ZfsConfig::default();
        let result = ZfsPoolManager::new(&config).await;
        // Should succeed even if ZFS is not available (runs in mock mode)
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_pool_manager_with_owned_config() {
        let config = ZfsConfig::default();
        let result = ZfsPoolManager::with_owned_config(config).await;
        // Should succeed even if ZFS is not available (runs in mock mode)
        assert!(result.is_ok());
    }

    // ==================== Size Parsing Tests ====================

    #[test]
    fn test_parse_size_bytes() {
        let _manager = ZfsPoolManager::new_for_testing();
        assert_eq!(
            parse_size_with_units("1024").expect("ZFS operation failed"),
            1024
        );
        assert_eq!(
            parse_size_with_units("512B").expect("ZFS operation failed"),
            512
        );
    }

    #[test]
    fn test_parse_size_kilobytes() {
        let _manager = ZfsPoolManager::new_for_testing();
        assert_eq!(
            parse_size_with_units("1K").expect("ZFS operation failed"),
            1024
        );
        assert_eq!(
            parse_size_with_units("10K").expect("ZFS operation failed"),
            10240
        );
    }

    #[test]
    fn test_parse_size_megabytes() {
        let _manager = ZfsPoolManager::new_for_testing();
        assert_eq!(
            parse_size_with_units("1M").expect("ZFS operation failed"),
            1_048_576
        );
        assert_eq!(
            parse_size_with_units("5M").expect("ZFS operation failed"),
            5_242_880
        );
    }

    #[test]
    fn test_parse_size_gigabytes() {
        let _manager = ZfsPoolManager::new_for_testing();
        assert_eq!(
            parse_size_with_units("1G").expect("ZFS operation failed"),
            1_073_741_824
        );
        assert_eq!(
            parse_size_with_units("2G").expect("ZFS operation failed"),
            2_147_483_648
        );
    }

    #[test]
    fn test_parse_size_terabytes() {
        let _manager = ZfsPoolManager::new_for_testing();
        assert_eq!(
            parse_size_with_units("1T").expect("ZFS operation failed"),
            1_099_511_627_776
        );
    }

    #[test]
    fn test_parse_size_petabytes() {
        let _manager = ZfsPoolManager::new_for_testing();
        assert_eq!(
            parse_size_with_units("1P").expect("ZFS operation failed"),
            1_125_899_906_842_624
        );
    }

    #[test]
    fn test_parse_size_dash() {
        let _manager = ZfsPoolManager::new_for_testing();
        assert_eq!(parse_size_with_units("-").expect("ZFS operation failed"), 0);
    }

    #[test]
    fn test_parse_size_invalid() {
        let _manager = ZfsPoolManager::new_for_testing();
        assert!(parse_size_with_units("invalid").is_none());
        assert!(parse_size_with_units("1X").is_none());
    }

    #[test]
    fn test_parse_size_decimal() {
        let _manager = ZfsPoolManager::new_for_testing();
        assert_eq!(
            parse_size_with_units("1.5G").expect("ZFS operation failed"),
            1_610_612_736
        );
        assert_eq!(
            parse_size_with_units("0.5M").expect("ZFS operation failed"),
            524_288
        );
    }

    // ==================== Pool Line Parsing Tests ====================

    #[test]
    fn test_parse_pool_line_valid() {
        let manager = ZfsPoolManager::new_for_testing();
        let line = "testpool\t10G\t5G\t5G\t50%\tONLINE";
        let result = manager.parse_pool_line(line).expect("ZFS operation failed");

        assert!(result.is_some());
        let pool = result.expect("ZFS operation failed");
        assert_eq!(pool.name, "testpool");
        assert_eq!(pool.state, PoolState::Online);
        assert_eq!(pool.health, PoolHealth::Healthy);
        assert_eq!(pool.capacity.utilization_percent, 50.0);
    }

    #[test]
    fn test_parse_pool_line_degraded() {
        let manager = ZfsPoolManager::new_for_testing();
        let line = "degraded_pool\t20G\t15G\t5G\t75%\tDEGRADED";
        let result = manager.parse_pool_line(line).expect("ZFS operation failed");

        assert!(result.is_some());
        let pool = result.expect("ZFS operation failed");
        assert_eq!(pool.name, "degraded_pool");
        assert_eq!(pool.state, PoolState::Degraded);
        assert_eq!(pool.health, PoolHealth::Warning);
    }

    #[test]
    fn test_parse_pool_line_faulted() {
        let manager = ZfsPoolManager::new_for_testing();
        let line = "faulted_pool\t100G\t50G\t50G\t50%\tFAULTED";
        let result = manager.parse_pool_line(line).expect("ZFS operation failed");

        assert!(result.is_some());
        let pool = result.expect("ZFS operation failed");
        assert_eq!(pool.name, "faulted_pool");
        assert_eq!(pool.state, PoolState::Faulted);
        assert_eq!(pool.health, PoolHealth::Critical);
    }

    #[test]
    fn test_parse_pool_line_offline() {
        let manager = ZfsPoolManager::new_for_testing();
        let line = "offline_pool\t50G\t0G\t50G\t0%\tOFFLINE";
        let result = manager.parse_pool_line(line).expect("ZFS operation failed");

        assert!(result.is_some());
        let pool = result.expect("ZFS operation failed");
        assert_eq!(pool.name, "offline_pool");
        assert_eq!(pool.state, PoolState::Offline);
    }

    #[test]
    fn test_parse_pool_line_insufficient_fields() {
        let manager = ZfsPoolManager::new_for_testing();
        let line = "incomplete\t10G";
        let result = manager.parse_pool_line(line).expect("ZFS operation failed");

        assert!(result.is_none());
    }

    #[test]
    fn test_parse_pool_line_unknown_health() {
        let manager = ZfsPoolManager::new_for_testing();
        let line = "unknown_pool\t10G\t5G\t5G\t50%\tUNKNOWN";
        let result = manager.parse_pool_line(line).expect("ZFS operation failed");

        assert!(result.is_some());
        let pool = result.expect("ZFS operation failed");
        assert_eq!(pool.health, PoolHealth::Unknown);
        assert_eq!(pool.state, PoolState::Unknown);
    }

    // ==================== Pool Capacity Tests ====================

    #[test]
    fn test_pool_capacity_zero_utilization() {
        let capacity = PoolCapacity {
            total_bytes: 1_000_000_000,
            used_bytes: 0,
            available_bytes: 1_000_000_000,
            utilization_percent: 0.0,
        };

        assert_eq!(capacity.utilization_percent, 0.0);
        assert_eq!(capacity.available_bytes, capacity.total_bytes);
    }

    #[test]
    fn test_pool_capacity_full_utilization() {
        let capacity = PoolCapacity {
            total_bytes: 1_000_000_000,
            used_bytes: 1_000_000_000,
            available_bytes: 0,
            utilization_percent: 100.0,
        };

        assert_eq!(capacity.utilization_percent, 100.0);
        assert_eq!(capacity.available_bytes, 0);
    }

    #[test]
    fn test_pool_capacity_partial_utilization() {
        let capacity = PoolCapacity {
            total_bytes: 1_000_000_000,
            used_bytes: 250_000_000,
            available_bytes: 750_000_000,
            utilization_percent: 25.0,
        };

        assert_eq!(
            capacity.used_bytes + capacity.available_bytes,
            capacity.total_bytes
        );
    }

    // ==================== Pool Info Serialization Tests ====================

    #[test]
    fn test_pool_info_serialization() {
        let pool = PoolInfo {
            name: "serialize_test".to_string(),
            state: PoolState::Online,
            health: PoolHealth::Healthy,
            capacity: PoolCapacity {
                total_bytes: 1000,
                used_bytes: 500,
                available_bytes: 500,
                utilization_percent: 50.0,
            },
            devices: vec![],
            properties: HashMap::new(),
        };

        let json = serde_json::to_string(&pool);
        assert!(json.is_ok());
    }

    #[test]
    fn test_pool_info_deserialization() {
        let json = r#"{
            "name": "deserialize_test",
            "state": "Online",
            "health": "Healthy",
            "capacity": {
                "total_bytes": 1000,
                "used_bytes": 500,
                "available_bytes": 500,
                "utilization_percent": 50.0
            },
            "devices": [],
            "properties": {}
        }"#;

        let pool: std::result::Result<PoolInfo, _> = serde_json::from_str(json);
        assert!(pool.is_ok());
        let pool = pool.expect("ZFS operation failed");
        assert_eq!(pool.name, "deserialize_test");
    }

    // ==================== Pool State Comparison Tests ====================

    #[test]
    fn test_pool_state_equality() {
        assert_eq!(PoolState::Online, PoolState::Online);
        assert_ne!(PoolState::Online, PoolState::Offline);
        assert_ne!(PoolState::Degraded, PoolState::Faulted);
    }

    #[test]
    fn test_pool_health_equality() {
        assert_eq!(PoolHealth::Healthy, PoolHealth::Healthy);
        assert_ne!(PoolHealth::Healthy, PoolHealth::Critical);
        assert_ne!(PoolHealth::Warning, PoolHealth::Unknown);
    }

    // ==================== Edge Case Tests ====================

    #[test]
    fn test_pool_info_with_many_devices() {
        let devices: Vec<String> = (0..100).map(|i| format!("/dev/sd{}", i)).collect();

        let pool = PoolInfo {
            name: "many_devices".to_string(),
            state: PoolState::Online,
            health: PoolHealth::Healthy,
            capacity: PoolCapacity {
                total_bytes: 100_000_000_000,
                used_bytes: 0,
                available_bytes: 100_000_000_000,
                utilization_percent: 0.0,
            },
            devices: devices.clone(),
            properties: HashMap::new(),
        };

        assert_eq!(pool.devices.len(), 100);
    }

    #[test]
    fn test_pool_info_with_properties() {
        let mut properties = HashMap::new();
        properties.insert("compression".to_string(), "lz4".to_string());
        properties.insert("dedup".to_string(), "off".to_string());
        properties.insert("atime".to_string(), "on".to_string());

        let pool = PoolInfo {
            name: "props_test".to_string(),
            state: PoolState::Online,
            health: PoolHealth::Healthy,
            capacity: PoolCapacity {
                total_bytes: 1000,
                used_bytes: 0,
                available_bytes: 1000,
                utilization_percent: 0.0,
            },
            devices: vec![],
            properties: properties.clone(),
        };

        assert_eq!(pool.properties.len(), 3);
        assert_eq!(pool.properties.get("compression"), Some(&"lz4".to_string()));
    }

    // ==================== Large Number Tests ====================

    #[test]
    fn test_pool_capacity_large_numbers() {
        let capacity = PoolCapacity {
            total_bytes: u64::MAX / 2,
            used_bytes: u64::MAX / 4,
            available_bytes: u64::MAX / 4,
            utilization_percent: 50.0,
        };

        assert!(capacity.total_bytes > 0);
        assert!(capacity.used_bytes > 0);
    }

    #[test]
    fn test_parse_size_max_values() {
        let _manager = ZfsPoolManager::new_for_testing();
        // Test large but valid sizes
        assert!(parse_size_with_units("9999P").is_some());
    }

    // ==================== Additional Pool Tests (Oct 18, 2025) ====================

    #[test]
    fn test_pool_capacity_calculation_consistency() {
        let total = 10_000_000_000u64;
        let used = 3_000_000_000u64;
        let available = total - used;

        let capacity = PoolCapacity {
            total_bytes: total,
            used_bytes: used,
            available_bytes: available,
            utilization_percent: (used as f64 / total as f64) * 100.0,
        };

        assert_eq!(capacity.total_bytes, total);
        assert_eq!(capacity.used_bytes, used);
        assert_eq!(capacity.available_bytes, available);
        assert_eq!(
            capacity.used_bytes + capacity.available_bytes,
            capacity.total_bytes
        );
    }

    #[test]
    fn test_pool_info_with_multiple_devices() {
        let devices = vec![
            "/dev/sda".to_string(),
            "/dev/sdb".to_string(),
            "/dev/sdc".to_string(),
        ];

        let pool = PoolInfo {
            name: "multi_device_pool".to_string(),
            state: PoolState::Online,
            health: PoolHealth::Healthy,
            capacity: PoolCapacity {
                total_bytes: 3_000_000_000,
                used_bytes: 1_000_000_000,
                available_bytes: 2_000_000_000,
                utilization_percent: 33.33,
            },
            devices: devices.clone(),
            properties: HashMap::new(),
        };

        assert_eq!(pool.devices.len(), 3);
        assert_eq!(pool.devices, devices);
    }

    #[test]
    fn test_pool_state_serialization() {
        // Test that pool states can be serialized
        let state = PoolState::Online;
        let json = serde_json::to_string(&state).expect("Failed to serialize");
        assert!(json.contains("Online"));
    }

    #[test]
    fn test_pool_capacity_serialization() {
        let capacity = PoolCapacity {
            total_bytes: 1000,
            used_bytes: 500,
            available_bytes: 500,
            utilization_percent: 50.0,
        };

        let json = serde_json::to_string(&capacity).expect("Failed to serialize");
        let deserialized: PoolCapacity =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(capacity.total_bytes, deserialized.total_bytes);
        assert_eq!(capacity.used_bytes, deserialized.used_bytes);
    }
}
