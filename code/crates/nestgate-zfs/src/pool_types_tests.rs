// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Comprehensive Tests for ZFS Pool Types
//!
//! Tests for pool information, state, health, and capacity types

#[cfg(test)]
mod tests {
    use crate::pool::types::{PoolCapacity, PoolHealth, PoolInfo, PoolState};
    use std::collections::HashMap;

    /// Test 1: PoolState enum variants
    #[test]
    fn test_pool_state_variants() {
        let states = [
            PoolState::Online,
            PoolState::Offline,
            PoolState::Degraded,
            PoolState::Faulted,
            PoolState::Unknown,
        ];

        assert_eq!(states.len(), 5);
        assert!(matches!(states[0], PoolState::Online));
        assert!(matches!(states[4], PoolState::Unknown));
    }

    /// Test 2: PoolState equality
    #[test]
    fn test_pool_state_equality() {
        assert_eq!(PoolState::Online, PoolState::Online);
        assert_ne!(PoolState::Online, PoolState::Offline);
        assert_eq!(PoolState::Degraded, PoolState::Degraded);
    }

    /// Test 3: PoolHealth enum variants
    #[test]
    fn test_pool_health_variants() {
        let health_states = [
            PoolHealth::Healthy,
            PoolHealth::Warning,
            PoolHealth::Critical,
            PoolHealth::Unknown,
        ];

        assert_eq!(health_states.len(), 4);
        assert!(matches!(health_states[0], PoolHealth::Healthy));
        assert!(matches!(health_states[2], PoolHealth::Critical));
    }

    /// Test 4: PoolHealth equality
    #[test]
    fn test_pool_health_equality() {
        assert_eq!(PoolHealth::Healthy, PoolHealth::Healthy);
        assert_ne!(PoolHealth::Healthy, PoolHealth::Warning);
        assert_eq!(PoolHealth::Critical, PoolHealth::Critical);
    }

    /// Test 5: PoolCapacity creation
    #[test]
    fn test_pool_capacity_creation() {
        let capacity = PoolCapacity {
            total_bytes: 1_000_000_000_000,   // 1TB
            used_bytes: 500_000_000_000,      // 500GB
            available_bytes: 500_000_000_000, // 500GB
            utilization_percent: 50.0,
            fragmentation_percent: 0.0,
            deduplication_ratio: 1.0,
            total: 1_000_000_000_000,
            used: 500_000_000_000,
            available: 500_000_000_000,
        };

        assert_eq!(capacity.total_bytes, 1_000_000_000_000);
        assert_eq!(capacity.used_bytes, 500_000_000_000);
        assert_eq!(capacity.available_bytes, 500_000_000_000);
        assert_eq!(capacity.utilization_percent, 50.0);
    }

    /// Test 6: PoolCapacity with zero values
    #[test]
    fn test_pool_capacity_empty() {
        let capacity = PoolCapacity {
            total_bytes: 0,
            used_bytes: 0,
            available_bytes: 0,
            utilization_percent: 0.0,
            fragmentation_percent: 0.0,
            deduplication_ratio: 1.0,
            total: 0,
            used: 0,
            available: 0,
        };

        assert_eq!(capacity.total_bytes, 0);
        assert_eq!(capacity.utilization_percent, 0.0);
    }

    /// Test 7: PoolCapacity with maximum values
    #[test]
    fn test_pool_capacity_maximum() {
        let capacity = PoolCapacity {
            total_bytes: u64::MAX,
            used_bytes: u64::MAX,
            available_bytes: 0,
            utilization_percent: 100.0,
            fragmentation_percent: 0.0,
            deduplication_ratio: 1.0,
            total: 0,
            used: 0,
            available: 0,
        };

        assert_eq!(capacity.total_bytes, u64::MAX);
        assert_eq!(capacity.used_bytes, u64::MAX);
        assert_eq!(capacity.utilization_percent, 100.0);
    }

    /// Test 8: PoolInfo creation
    #[test]
    fn test_pool_info_creation() {
        let mut properties = HashMap::new();
        properties.insert("compression".to_string(), "lz4".to_string());
        properties.insert("dedup".to_string(), "off".to_string());

        let pool = PoolInfo {
            name: "tank".to_string(),
            state: PoolState::Online,
            health: PoolHealth::Healthy,
            capacity: PoolCapacity {
                total_bytes: 1_000_000_000_000,
                used_bytes: 100_000_000_000,
                available_bytes: 900_000_000_000,
                utilization_percent: 10.0,
                fragmentation_percent: 0.0,
                deduplication_ratio: 1.0,
                total: 1_000_000_000_000,
                used: 100_000_000_000,
                available: 900_000_000_000,
            },
            devices: vec!["/dev/sda".to_string(), "/dev/sdb".to_string()],
            properties: properties.clone(),
        };

        assert_eq!(pool.name, "tank");
        assert_eq!(pool.state, PoolState::Online);
        assert_eq!(pool.health, PoolHealth::Healthy);
        assert_eq!(pool.devices.len(), 2);
        assert_eq!(pool.properties.len(), 2);
        assert_eq!(pool.properties.get("compression").unwrap(), "lz4");
    }

    /// Test 9: PoolInfo with no devices
    #[test]
    fn test_pool_info_no_devices() {
        let pool = PoolInfo {
            name: "empty_pool".to_string(),
            state: PoolState::Unknown,
            health: PoolHealth::Unknown,
            capacity: PoolCapacity {
                total_bytes: 0,
                used_bytes: 0,
                available_bytes: 0,
                utilization_percent: 0.0,
                fragmentation_percent: 0.0,
                deduplication_ratio: 1.0,
                total: 0,
                used: 0,
                available: 0,
            },
            devices: vec![],
            properties: HashMap::new(),
        };

        assert!(pool.devices.is_empty());
        assert!(pool.properties.is_empty());
    }

    /// Test 10: PoolInfo with many devices
    #[test]
    fn test_pool_info_many_devices() {
        let devices: Vec<String> = (0..100)
            .map(|i| format!("/dev/sd{}", (b'a' + i as u8) as char))
            .collect();

        let pool = PoolInfo {
            name: "large_pool".to_string(),
            state: PoolState::Online,
            health: PoolHealth::Healthy,
            capacity: PoolCapacity {
                total_bytes: 100_000_000_000_000, // 100TB
                used_bytes: 50_000_000_000_000,
                available_bytes: 50_000_000_000_000,
                utilization_percent: 50.0,
                fragmentation_percent: 0.0,
                deduplication_ratio: 1.0,
                total: 100_000_000_000_000,
                used: 50_000_000_000_000,
                available: 50_000_000_000_000,
            },
            devices: devices.clone(),
            properties: HashMap::new(),
        };

        assert_eq!(pool.devices.len(), 100);
        assert_eq!(pool.devices[0], "/dev/sda");
    }

    /// Test 11: PoolInfo cloning
    #[test]
    fn test_pool_info_clone() {
        let pool = PoolInfo {
            name: "original".to_string(),
            state: PoolState::Online,
            health: PoolHealth::Healthy,
            capacity: PoolCapacity {
                total_bytes: 1_000_000_000,
                used_bytes: 500_000_000,
                available_bytes: 500_000_000,
                utilization_percent: 50.0,
                fragmentation_percent: 0.0,
                deduplication_ratio: 1.0,
                total: 1_000_000_000,
                used: 500_000_000,
                available: 500_000_000,
            },
            devices: vec!["/dev/sda".to_string()],
            properties: HashMap::new(),
        };

        let cloned = pool.clone();
        assert_eq!(cloned.name, pool.name);
        assert_eq!(cloned.state, pool.state);
        assert_eq!(cloned.health, pool.health);
        assert_eq!(cloned.devices.len(), pool.devices.len());
    }

    /// Test 12: PoolCapacity calculation validation
    #[test]
    fn test_pool_capacity_consistency() {
        let capacity = PoolCapacity {
            total_bytes: 1_000_000_000,
            used_bytes: 300_000_000,
            available_bytes: 700_000_000,
            utilization_percent: 30.0,
            fragmentation_percent: 0.0,
            deduplication_ratio: 1.0,
            total: 1_000_000_000,
            used: 300_000_000,
            available: 700_000_000,
        };

        // Verify used + available equals total
        assert_eq!(
            capacity.used_bytes + capacity.available_bytes,
            capacity.total_bytes
        );

        // Verify utilization percentage is reasonable
        assert!(capacity.utilization_percent >= 0.0 && capacity.utilization_percent <= 100.0);
    }

    /// Test 13: PoolState serialization support
    #[test]
    fn test_pool_state_debug() {
        let state = PoolState::Degraded;
        let debug_str = format!("{:?}", state);
        assert!(debug_str.contains("Degraded"));
    }

    /// Test 14: PoolHealth serialization support
    #[test]
    fn test_pool_health_debug() {
        let health = PoolHealth::Warning;
        let debug_str = format!("{:?}", health);
        assert!(debug_str.contains("Warning"));
    }

    /// Test 15: PoolInfo with special characters in name
    #[test]
    fn test_pool_info_special_name() {
        let pool = PoolInfo {
            name: "tank-backup_2025".to_string(),
            state: PoolState::Online,
            health: PoolHealth::Healthy,
            capacity: PoolCapacity {
                total_bytes: 1_000_000_000,
                used_bytes: 0,
                available_bytes: 1_000_000_000,
                utilization_percent: 0.0,
                fragmentation_percent: 0.0,
                deduplication_ratio: 1.0,
                total: 1_000_000_000,
                used: 0,
                available: 1_000_000_000,
            },
            devices: vec![],
            properties: HashMap::new(),
        };

        assert!(pool.name.contains("-"));
        assert!(pool.name.contains("_"));
        assert!(pool.name.contains("2025"));
    }

    /// Test 16: PoolInfo with many properties
    #[test]
    fn test_pool_info_many_properties() {
        let mut properties = HashMap::new();
        for i in 0..50 {
            properties.insert(format!("prop{}", i), format!("value{}", i));
        }

        let pool = PoolInfo {
            name: "props_pool".to_string(),
            state: PoolState::Online,
            health: PoolHealth::Healthy,
            capacity: PoolCapacity {
                total_bytes: 1_000_000_000,
                used_bytes: 0,
                available_bytes: 1_000_000_000,
                utilization_percent: 0.0,
                fragmentation_percent: 0.0,
                deduplication_ratio: 1.0,
                total: 1_000_000_000,
                used: 0,
                available: 1_000_000_000,
            },
            devices: vec![],
            properties: properties.clone(),
        };

        assert_eq!(pool.properties.len(), 50);
        assert!(pool.properties.contains_key("prop0"));
        assert!(pool.properties.contains_key("prop49"));
    }

    /// Test 17: PoolCapacity utilization edge cases
    #[test]
    fn test_pool_capacity_edge_cases() {
        // Fully utilized
        let full = PoolCapacity {
            total_bytes: 1_000_000_000,
            used_bytes: 1_000_000_000,
            available_bytes: 0,
            utilization_percent: 100.0,
            fragmentation_percent: 0.0,
            deduplication_ratio: 1.0,
            total: 1_000_000_000,
            used: 1_000_000_000,
            available: 0,
        };
        assert_eq!(full.utilization_percent, 100.0);

        // Empty
        let empty = PoolCapacity {
            total_bytes: 1_000_000_000,
            used_bytes: 0,
            available_bytes: 1_000_000_000,
            utilization_percent: 0.0,
            fragmentation_percent: 0.0,
            deduplication_ratio: 1.0,
            total: 1_000_000_000,
            used: 0,
            available: 1_000_000_000,
        };
        assert_eq!(empty.utilization_percent, 0.0);
    }

    /// Test 18: PoolState exhaustive matching
    #[test]
    fn test_pool_state_exhaustive() {
        let test_state = |state: PoolState| -> &str {
            match state {
                PoolState::Online => "online",
                PoolState::Offline => "offline",
                PoolState::Degraded => "degraded",
                PoolState::Faulted => "faulted",
                PoolState::Unknown => "unknown",
            }
        };

        assert_eq!(test_state(PoolState::Online), "online");
        assert_eq!(test_state(PoolState::Faulted), "faulted");
    }

    /// Test 19: PoolHealth exhaustive matching
    #[test]
    fn test_pool_health_exhaustive() {
        let test_health = |health: PoolHealth| -> &str {
            match health {
                PoolHealth::Healthy => "healthy",
                PoolHealth::Warning => "warning",
                PoolHealth::Critical => "critical",
                PoolHealth::Unknown => "unknown",
            }
        };

        assert_eq!(test_health(PoolHealth::Healthy), "healthy");
        assert_eq!(test_health(PoolHealth::Critical), "critical");
    }

    /// Test 20: PoolInfo with unicode in properties
    #[test]
    fn test_pool_info_unicode_properties() {
        let mut properties = HashMap::new();
        properties.insert("description".to_string(), "데이터 풀".to_string()); // Korean
        properties.insert("location".to_string(), "東京".to_string()); // Japanese

        let pool = PoolInfo {
            name: "unicode_pool".to_string(),
            state: PoolState::Online,
            health: PoolHealth::Healthy,
            capacity: PoolCapacity {
                total_bytes: 1_000_000_000,
                used_bytes: 0,
                available_bytes: 1_000_000_000,
                utilization_percent: 0.0,
                fragmentation_percent: 0.0,
                deduplication_ratio: 1.0,
                total: 1_000_000_000,
                used: 0,
                available: 1_000_000_000,
            },
            devices: vec![],
            properties: properties.clone(),
        };

        assert_eq!(pool.properties.get("description").unwrap(), "데이터 풀");
        assert_eq!(pool.properties.get("location").unwrap(), "東京");
    }
}
