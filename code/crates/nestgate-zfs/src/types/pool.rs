//! Pool-related types for ZFS
//!
//! This module contains all types related to ZFS pools, including pool information,
//! health status, state, and capacity metrics.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

use super::common::ZfsError;

/// Complete information about a ZFS storage pool
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
    /// Current health status
    pub health: PoolHealth,
    /// Current operational state
    pub state: PoolState,
    /// Detailed capacity information
    pub capacity: PoolCapacity,
    /// Custom ZFS properties
    pub properties: HashMap<String, String>,
    /// When the pool was created
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

/// ZFS pool health status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PoolHealth {
    /// Pool is healthy and fully operational
    Healthy,
    /// Pool has warnings but is still operational
    Warning,
    /// Pool is in critical state, data may be at risk
    Critical,
    /// Pool health status cannot be determined
    Unknown,
}

/// ZFS pool operational state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PoolState {
    /// Pool is online and accessible
    Online,
    /// Pool is offline and not accessible
    Offline,
    /// Pool is degraded (reduced redundancy)
    Degraded,
    /// Pool has faulted devices
    Faulted,
    /// Pool has been removed
    Removed,
    /// Pool is temporarily unavailable
    Unavailable,
}

/// ZFS pool capacity information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolCapacity {
    /// Total capacity in bytes
    pub total_bytes: u64,
    /// Used capacity in bytes
    pub used_bytes: u64,
    /// Available capacity in bytes
    pub available_bytes: u64,
    /// Fragmentation percentage (0.0-100.0)
    pub fragmentation_percent: f64,
    /// Deduplication ratio (>1.0 = space saved)
    pub deduplication_ratio: f64,
}

impl Default for PoolCapacity {
    fn default() -> Self {
        Self {
            total_bytes: 0,
            used_bytes: 0,
            available_bytes: 0,
            fragmentation_percent: 0.0,
            deduplication_ratio: 1.0,
        }
    }
}

/// Pool status for health checks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PoolStatus {
    /// Pool is healthy
    Healthy,
    /// Pool is degraded
    Degraded,
    /// Pool has faulted
    Faulted,
    /// Pool is offline
    Offline,
    /// Pool is removed
    Removed,
    /// Pool is unavailable
    Unavailable,
    /// Status unknown
    Unknown,
}

// Conversions
impl From<PoolStatus> for PoolHealth {
    fn from(status: PoolStatus) -> Self {
        match status {
            PoolStatus::Healthy => PoolHealth::Healthy,
            PoolStatus::Degraded => PoolHealth::Warning,
            PoolStatus::Faulted => PoolHealth::Critical,
            PoolStatus::Offline | PoolStatus::Removed | PoolStatus::Unavailable | PoolStatus::Unknown => PoolHealth::Unknown,
        }
    }
}

impl From<PoolStatus> for PoolState {
    fn from(status: PoolStatus) -> Self {
        match status {
            PoolStatus::Healthy => PoolState::Online,
            PoolStatus::Degraded => PoolState::Degraded,
            PoolStatus::Faulted => PoolState::Faulted,
            PoolStatus::Offline => PoolState::Offline,
            PoolStatus::Removed => PoolState::Removed,
            PoolStatus::Unavailable | PoolStatus::Unknown => PoolState::Unavailable,
        }
    }
}

/// Zero-cost pool info using references and Arc for sharing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroCostPoolInfo {
    /// Pool name
    pub name: String,
    /// Total size in bytes
    pub size: u64,
    /// Used size in bytes
    pub used: u64,
    /// Available size in bytes
    pub available: u64,
    /// Health status
    pub health: String,
}

