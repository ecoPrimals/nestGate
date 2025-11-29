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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
/// Poolcapacity
pub struct PoolCapacity {
    /// Total Bytes
    pub total_bytes: u64,
    /// Used Bytes
    pub used_bytes: u64,
    /// Available Bytes
    pub available_bytes: u64,
    /// Utilization Percent
    pub utilization_percent: f64,
}
