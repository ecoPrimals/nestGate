/// Core types and data structures for NestGate
///
/// This module contains fundamental data types used throughout the system.
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

/// Allocation status for resources
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AllocationStatus {
    Active,
    Inactive,
    Pending,
    Failed,
}
/// Storage tier enum for tiered storage management
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StorageTier {
    /// High-performance storage for frequently accessed data
    Hot,
    /// Medium-performance storage for moderately accessed data
    Warm,
    /// Low-performance storage for rarely accessed data
    Cold,
    /// Fast cache storage for temporary data
    Cache,
    /// Long-term archival storage for rarely accessed data
    Archive,
}
impl Display for StorageTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageTier::Hot => write!(f, "Hot"),
            StorageTier::Warm => write!(f, "Warm"),
            StorageTier::Cold => write!(f, "Cold"),
            StorageTier::Cache => write!(f, "Cache"),
            StorageTier::Archive => write!(f, "Archive"),
        }
    }
}

impl StorageTier {
    /// Get all available storage tiers
    pub fn all() -> Vec<StorageTier> {
        vec![
            Self::Hot,
            Self::Warm,
            Self::Cold,
            Self::Cache,
            Self::Archive,
        ]
    }

    /// Get the priority order of tiers (Hot = highest priority)
    pub fn priority(&self) -> u8 {
        match self {
            StorageTier::Hot => 0,
            StorageTier::Warm => 1,
            StorageTier::Cold => 2,
            StorageTier::Cache => 3,
            StorageTier::Archive => 4,
        }
    }

    /// Get string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            StorageTier::Hot => "hot",
            StorageTier::Warm => "warm",
            StorageTier::Cold => "cold",
            StorageTier::Cache => "cache",
            StorageTier::Archive => "archive",
        }
    }
}

impl Default for StorageTier {
    fn default() -> Self {
        Self::Hot
    }
}

/// Health status for system components
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}
impl Default for HealthStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Display for HealthStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HealthStatus::Healthy => write!(f, "Healthy"),
            HealthStatus::Degraded => write!(f, "Degraded"),
            HealthStatus::Unhealthy => write!(f, "Unhealthy"),
            HealthStatus::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Service state for tracking service status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ServiceState {
    Running,
    Stopped,
    Starting,
    Stopping,
    Error,
    Unknown,
}
impl Default for ServiceState {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Display for ServiceState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceState::Running => write!(f, "Running"),
            ServiceState::Stopped => write!(f, "Stopped"),
            ServiceState::Starting => write!(f, "Starting"),
            ServiceState::Stopping => write!(f, "Stopping"),
            ServiceState::Error => write!(f, "Error"),
            ServiceState::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Performance tier for storage and compute resources
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PerformanceTier {
    Ultra,
    High,
    Standard,
    Economy,
}
impl Default for PerformanceTier {
    fn default() -> Self {
        Self::Standard
    }
}

impl Display for PerformanceTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PerformanceTier::Ultra => write!(f, "Ultra"),
            PerformanceTier::High => write!(f, "High"),
            PerformanceTier::Standard => write!(f, "Standard"),
            PerformanceTier::Economy => write!(f, "Economy"),
        }
    }
}
