// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! ZFS pool-related types
//!
//! Domain: Pool information, health status, capacity, vdev management

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// ZFS pool information and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolInfo {
    /// Pool name
    pub name: String,
    /// Total pool size in bytes
    pub size: u64,
    /// Used space in bytes
    pub used: u64,
    /// Available space in bytes
    pub available: u64,
    /// Pool health status
    pub health: PoolHealth,
    /// Current pool state
    pub state: PoolState,
    /// Detailed capacity information
    pub capacity: PoolCapacity,
    /// ZFS properties for this pool
    pub properties: HashMap<String, String>,
    /// Pool creation timestamp
    pub created_at: SystemTime,
}

impl Default for PoolInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            size: 0,
            used: 0,
            available: 0,
            health: PoolHealth::Unknown,
            state: PoolState::Offline,
            capacity: PoolCapacity::default(),
            properties: HashMap::new(),
            created_at: SystemTime::now(),
        }
    }
}

/// ZFS pool health status indicators
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PoolHealth {
    /// Pool is operating normally
    Healthy,
    /// Pool has warnings but is functional
    Warning,
    /// Pool has critical issues
    Critical,
    /// Pool health status unknown
    Unknown,
}

/// ZFS pool operational state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PoolState {
    /// Pool is online and available
    Online,
    /// Pool is offline
    Offline,
    /// Pool is degraded but operational
    Degraded,
    /// Pool has faulted
    Faulted,
    /// Pool device has been removed
    Removed,
    /// Pool is unavailable
    Unavailable,
}

/// Detailed pool capacity information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolCapacity {
    /// Total capacity in bytes
    pub total: u64,
    /// Alternative field name for backward compatibility
    pub total_bytes: u64,
    /// Used capacity in bytes
    pub used: u64,
    /// Alternative field name for backward compatibility
    pub used_bytes: u64,
    /// Available capacity in bytes
    pub available: u64,
    /// Alternative field name for backward compatibility
    pub available_bytes: u64,
    /// Capacity utilization as percentage (0.0-100.0)
    pub utilization_percent: f64,
    /// Fragmentation percentage (0.0-100.0)
    pub fragmentation_percent: f64,
    /// Deduplication ratio (e.g., 1.5 means 50% space savings)
    pub deduplication_ratio: f64,
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

/// Pool status for monitoring and health checks
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PoolStatus {
    /// Pool is operating normally
    Healthy,
    /// Pool is online and fully operational
    Online,
    /// Pool is degraded but operational
    Degraded,
    /// Pool has critical issues
    Critical,
    /// Pool has faulted and requires intervention
    Faulted,
    /// Pool is offline or unavailable
    Offline,
    /// Pool device has been removed
    Removed,
    /// Pool is unavailable
    Unavailable,
    /// Pool status is unknown
    Unknown,
}

impl From<PoolStatus> for PoolHealth {
    fn from(status: PoolStatus) -> Self {
        match status {
            PoolStatus::Healthy | PoolStatus::Online => Self::Healthy,
            PoolStatus::Degraded => Self::Warning,
            PoolStatus::Critical | PoolStatus::Faulted => Self::Critical,
            PoolStatus::Offline
            | PoolStatus::Removed
            | PoolStatus::Unavailable
            | PoolStatus::Unknown => Self::Unknown,
        }
    }
}

impl From<PoolStatus> for PoolState {
    fn from(status: PoolStatus) -> Self {
        match status {
            PoolStatus::Online | PoolStatus::Healthy => Self::Online,
            PoolStatus::Degraded => Self::Degraded,
            PoolStatus::Faulted | PoolStatus::Critical => Self::Faulted,
            PoolStatus::Offline => Self::Offline,
            PoolStatus::Removed => Self::Removed,
            PoolStatus::Unavailable | PoolStatus::Unknown => Self::Unavailable,
        }
    }
}

/// Virtual device (vdev) information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VdevInfo {
    /// Vdev type (disk, mirror, raidz, etc.)
    pub vdev_type: String,
    /// Device path or identifier
    pub path: String,
    /// Vdev state
    pub state: String,
    /// Number of read errors
    pub read_errors: u64,
    /// Number of write errors
    pub write_errors: u64,
    /// Number of checksum errors
    pub checksum_errors: u64,
}

/// Pool statistics for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolStats {
    /// Pool name
    pub pool_name: String,
    /// Total read operations
    pub read_ops: u64,
    /// Total write operations
    pub write_ops: u64,
    /// Bytes read
    pub bytes_read: u64,
    /// Bytes written
    pub bytes_written: u64,
    /// Timestamp of statistics collection
    pub timestamp: SystemTime,
}

#[cfg(test)]
#[expect(
    clippy::float_cmp,
    reason = "Default() tests compare exact sentinel ratios (1.0 dedup) and 0.0 utilization"
)]
mod round5_pool_impl_tests {
    use super::*;

    #[test]
    fn round5_pool_info_default_fields() {
        let p = PoolInfo::default();
        assert!(p.name.is_empty());
        assert_eq!(p.health, PoolHealth::Unknown);
        assert_eq!(p.capacity.deduplication_ratio, 1.0);
    }

    #[test]
    fn round5_pool_capacity_default_impl() {
        let c = PoolCapacity::default();
        assert_eq!(c.total, 0);
        assert_eq!(c.utilization_percent, 0.0);
    }

    #[test]
    fn round5_pool_status_to_health_impl() {
        let h: PoolHealth = PoolStatus::Healthy.into();
        assert_eq!(h, PoolHealth::Healthy);
        let h: PoolHealth = PoolStatus::Degraded.into();
        assert_eq!(h, PoolHealth::Warning);
    }

    #[test]
    fn round5_pool_status_to_state_impl() {
        let s: PoolState = PoolStatus::Online.into();
        assert!(matches!(s, PoolState::Online));
        let s: PoolState = PoolStatus::Faulted.into();
        assert!(matches!(s, PoolState::Faulted));
    }

    #[test]
    fn round5_pool_health_serde_roundtrip() {
        let h = PoolHealth::Critical;
        let json = serde_json::to_string(&h).unwrap();
        let back: PoolHealth = serde_json::from_str(&json).unwrap();
        assert_eq!(h, back);
    }

    #[test]
    fn round5_pool_info_serde_roundtrip() {
        let p = PoolInfo {
            name: "tank".to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&p).unwrap();
        let back: PoolInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(back.name, "tank");
    }
}
