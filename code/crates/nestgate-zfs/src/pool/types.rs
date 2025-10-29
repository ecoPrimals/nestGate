//! ZFS Pool Types
//!
//! Type definitions for ZFS pool management

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Information about a discovered ZFS pool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolInfo {
    pub name: String,
    pub state: PoolState,
    pub health: PoolHealth,
    pub capacity: PoolCapacity,
    pub devices: Vec<String>,
    pub properties: HashMap<String, String>,
}

/// ZFS Pool State
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PoolState {
    Online,
    Offline,
    Degraded,
    Faulted,
    Unknown,
}

/// ZFS Pool Health Status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PoolHealth {
    Healthy,
    Warning,
    Critical,
    Unknown,
}

/// ZFS Pool Capacity Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolCapacity {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub utilization_percent: f64,
}
