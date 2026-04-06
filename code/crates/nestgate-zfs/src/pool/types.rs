// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! ZFS Pool Types
//!
//! Type definitions for ZFS pool management

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Information about a discovered ZFS pool
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Poolinfo
pub struct PoolInfo {
    /// Name
    pub name: String,
    /// State
    pub state: PoolState,
    /// Health
    pub health: PoolHealth,
    /// Capacity
    pub capacity: PoolCapacity,
    /// Devices
    pub devices: Vec<String>,
    /// Properties
    pub properties: HashMap<String, String>,
}

/// ZFS Pool State
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Poolstate
pub enum PoolState {
    /// Online
    Online,
    /// Offline
    Offline,
    /// Degraded
    Degraded,
    /// Faulted
    Faulted,
    /// Unknown
    Unknown,
}

/// ZFS Pool Health Status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Poolhealth
pub enum PoolHealth {
    /// Healthy
    Healthy,
    /// Warning
    Warning,
    /// Critical
    Critical,
    /// Unknown
    Unknown,
}

/// ZFS Pool Capacity Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolCapacity {
    /// Total capacity in bytes
    pub total: u64,
    /// Total Bytes (backward compatibility)
    pub total_bytes: u64,
    /// Used capacity in bytes
    pub used: u64,
    /// Used Bytes (backward compatibility)
    pub used_bytes: u64,
    /// Available capacity in bytes
    pub available: u64,
    /// Available Bytes (backward compatibility)
    pub available_bytes: u64,
    /// Utilization Percent
    pub utilization_percent: f64,
    /// Fragmentation percentage
    #[serde(default)]
    pub fragmentation_percent: f64,
    /// Deduplication ratio
    #[serde(default = "default_dedup_ratio")]
    pub deduplication_ratio: f64,
}

const fn default_dedup_ratio() -> f64 {
    1.0
}

impl Default for PoolCapacity {
    fn default() -> Self {
        Self {
            total: 0,
            total_bytes: 0,
            used: 0,
            used_bytes: 0,
            available: 0,
            available_bytes: 0,
            utilization_percent: 0.0,
            fragmentation_percent: 0.0,
            deduplication_ratio: 1.0,
        }
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn pool_info_serde_and_enums() {
        let info = PoolInfo {
            name: "tank".to_string(),
            state: PoolState::Online,
            health: PoolHealth::Healthy,
            capacity: PoolCapacity::default(),
            devices: vec!["/dev/sda".to_string()],
            properties: HashMap::from([("key".to_string(), "v".to_string())]),
        };
        let json = serde_json::to_string(&info).expect("serialize");
        let back: PoolInfo = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.name, "tank");
        assert_eq!(back.state, PoolState::Online);

        for s in [
            PoolState::Online,
            PoolState::Offline,
            PoolState::Degraded,
            PoolState::Faulted,
            PoolState::Unknown,
        ] {
            let v = serde_json::to_string(&s).unwrap();
            let _: PoolState = serde_json::from_str(&v).unwrap();
        }
        for h in [
            PoolHealth::Healthy,
            PoolHealth::Warning,
            PoolHealth::Critical,
            PoolHealth::Unknown,
        ] {
            let v = serde_json::to_string(&h).unwrap();
            let _: PoolHealth = serde_json::from_str(&v).unwrap();
        }
    }

    #[test]
    fn pool_capacity_default_and_serde() {
        let c = PoolCapacity::default();
        assert_eq!(c.deduplication_ratio, 1.0);
        let json = serde_json::to_string(&c).unwrap();
        let _: PoolCapacity = serde_json::from_str(&json).unwrap();
    }
}
