// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

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
