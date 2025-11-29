//! ZFS Pool Management Types
//!
//! Types for managing ZFS pools including health, state, capacity, and scrub status.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Information about a ZFS pool
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Poolinfo
pub struct PoolInfo {
    /// Pool name
    pub name: String,
    /// Pool health status
    pub health: PoolHealth,
    /// Pool state
    pub state: PoolState,
    /// Pool capacity information
    pub capacity: PoolCapacity,
    /// Scrub status
    pub scrub: Option<ScrubStatus>,
    /// Additional properties
    pub properties: HashMap<String, String>,
}

/// Pool health status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Poolhealth
pub enum PoolHealth {
    /// Pool is healthy
    Online,
    /// Pool has degraded devices
    Degraded,
    /// Pool is faulted
    Faulted,
    /// Pool is offline
    Offline,
    /// Pool health is unknown
    Unknown,
}

/// Pool state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Poolstate
pub enum PoolState {
    /// Pool is active
    Active,
    /// Pool is exported
    Exported,
    /// Pool is destroyed
    Destroyed,
    /// Pool is suspended
    Suspended,
    /// Pool state is unknown
    Unknown,
}

/// Pool capacity information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Poolcapacity
pub struct PoolCapacity {
    /// Total capacity in bytes
    pub total: u64,
    /// Used space in bytes
    pub used: u64,
    /// Available space in bytes
    pub available: u64,
}

/// Scrub status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Status values for Scrub
pub enum ScrubStatus {
    /// No scrub in progress
    None,
    /// Scrub in progress
    InProgress,
    /// Scrub completed
    Completed,
    /// Scrub cancelled
    Cancelled,
}

/// Pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Pool
pub struct PoolConfig {
    /// Pool name
    pub name: String,
    /// Devices for the pool (disk paths)
    #[serde(default)]
    ///  Devices
    pub _devices: Vec<String>,
    /// Mount point
    pub mountpoint: Option<String>,
    /// Compression enabled
    pub compression: bool,
    /// Deduplication enabled
    pub deduplication: bool,
    /// Additional properties
    pub properties: HashMap<String, String>,
}
