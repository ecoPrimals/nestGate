/// Core types and data structures for NestGate
///
/// This module contains fundamental data types used throughout the system.
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

#[cfg(test)]
mod types_tests;

/// Allocation status for resources
///
/// Represents the current state of a resource allocation in the system.
/// Used to track whether resources are actively in use, waiting, or have failed.
///
/// # Variants
///
/// * `Active` - Resource is currently allocated and in use
/// * `Inactive` - Resource is allocated but not currently in use
/// * `Pending` - Resource allocation is pending/in progress
/// * `Failed` - Resource allocation has failed
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Status values for Allocation
pub enum AllocationStatus {
    /// Resource is currently allocated and in use
    Active,
    /// Resource is allocated but not currently in use
    Inactive,
    /// Resource allocation is pending/in progress
    Pending,
    /// Resource allocation has failed
    Failed,
}
/// Storage tier enum for tiered storage management
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Storagetier
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
    /// Fmt
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
    /// Returns the default instance
    fn default() -> Self {
        Self::Hot
    }
}

/// Health status for system components
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Status values for Health
pub enum HealthStatus {
    /// Healthy
    Healthy,
    /// Degraded
    Degraded,
    /// Unhealthy
    Unhealthy,
    /// Unknown
    Unknown,
}
impl Default for HealthStatus {
    /// Returns the default instance
    fn default() -> Self {
        Self::Unknown
    }
}

impl Display for HealthStatus {
    /// Fmt
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
/// Servicestate
pub enum ServiceState {
    /// Running
    Running,
    /// Stopped
    Stopped,
    /// Starting
    Starting,
    /// Stopping
    Stopping,
    /// Error
    Error,
    /// Unknown
    Unknown,
}
impl Default for ServiceState {
    /// Returns the default instance
    fn default() -> Self {
        Self::Unknown
    }
}

impl Display for ServiceState {
    /// Fmt
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
/// Performancetier
pub enum PerformanceTier {
    /// Ultra
    Ultra,
    /// High
    High,
    /// Standard
    Standard,
    /// Economy
    Economy,
}
impl Default for PerformanceTier {
    /// Returns the default instance
    fn default() -> Self {
        Self::Standard
    }
}

impl Display for PerformanceTier {
    /// Fmt
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PerformanceTier::Ultra => write!(f, "Ultra"),
            PerformanceTier::High => write!(f, "High"),
            PerformanceTier::Standard => write!(f, "Standard"),
            PerformanceTier::Economy => write!(f, "Economy"),
        }
    }
}
