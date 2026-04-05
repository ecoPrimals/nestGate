// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

/// Core types and data structures for NestGate
///
/// This module contains fundamental data types used throughout the system.
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

// Tests moved to tests/comprehensive_type_tests.rs for better organization
// This complies with 1000-line limit and separates concerns

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
///
/// Status values for Allocation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
            Self::Hot => write!(f, "Hot"),
            Self::Warm => write!(f, "Warm"),
            Self::Cold => write!(f, "Cold"),
            Self::Cache => write!(f, "Cache"),
            Self::Archive => write!(f, "Archive"),
        }
    }
}

impl StorageTier {
    /// Get all available storage tiers
    #[must_use]
    pub fn all() -> Vec<Self> {
        vec![
            Self::Hot,
            Self::Warm,
            Self::Cold,
            Self::Cache,
            Self::Archive,
        ]
    }

    /// Get the priority order of tiers (Hot = highest priority)
    #[must_use]
    pub const fn priority(&self) -> u8 {
        match self {
            Self::Hot => 0,
            Self::Warm => 1,
            Self::Cold => 2,
            Self::Cache => 3,
            Self::Archive => 4,
        }
    }

    /// Get string representation
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Hot => "hot",
            Self::Warm => "warm",
            Self::Cold => "cold",
            Self::Cache => "cache",
            Self::Archive => "archive",
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
            Self::Healthy => write!(f, "Healthy"),
            Self::Degraded => write!(f, "Degraded"),
            Self::Unhealthy => write!(f, "Unhealthy"),
            Self::Unknown => write!(f, "Unknown"),
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
            Self::Running => write!(f, "Running"),
            Self::Stopped => write!(f, "Stopped"),
            Self::Starting => write!(f, "Starting"),
            Self::Stopping => write!(f, "Stopping"),
            Self::Error => write!(f, "Error"),
            Self::Unknown => write!(f, "Unknown"),
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
            Self::Ultra => write!(f, "Ultra"),
            Self::High => write!(f, "High"),
            Self::Standard => write!(f, "Standard"),
            Self::Economy => write!(f, "Economy"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocation_status_variants() {
        let _active = AllocationStatus::Active;
        let _inactive = AllocationStatus::Inactive;
        let _pending = AllocationStatus::Pending;
        let _failed = AllocationStatus::Failed;
        assert!(matches!(AllocationStatus::Active, AllocationStatus::Active));
    }

    #[test]
    fn test_allocation_status_serialization() {
        let status = AllocationStatus::Active;
        let json = serde_json::to_string(&status).unwrap();
        let parsed: AllocationStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(status, parsed);
    }

    #[test]
    fn test_storage_tier_all() {
        let tiers = StorageTier::all();
        assert_eq!(tiers.len(), 5);
        assert!(tiers.contains(&StorageTier::Hot));
        assert!(tiers.contains(&StorageTier::Archive));
    }

    #[test]
    fn test_storage_tier_priority() {
        assert_eq!(StorageTier::Hot.priority(), 0);
        assert_eq!(StorageTier::Warm.priority(), 1);
        assert_eq!(StorageTier::Archive.priority(), 4);
    }

    #[test]
    fn test_storage_tier_as_str() {
        assert_eq!(StorageTier::Hot.as_str(), "hot");
        assert_eq!(StorageTier::Cache.as_str(), "cache");
    }

    #[test]
    fn test_storage_tier_display() {
        assert_eq!(StorageTier::Hot.to_string(), "Hot");
        assert_eq!(StorageTier::Cold.to_string(), "Cold");
    }

    #[test]
    fn test_storage_tier_default() {
        assert_eq!(StorageTier::default(), StorageTier::Hot);
    }

    #[test]
    fn test_health_status_variants() {
        assert_eq!(HealthStatus::Healthy.to_string(), "Healthy");
        assert_eq!(HealthStatus::Degraded.to_string(), "Degraded");
        assert_eq!(HealthStatus::Unknown.to_string(), "Unknown");
    }

    #[test]
    fn test_health_status_default() {
        assert_eq!(HealthStatus::default(), HealthStatus::Unknown);
    }

    #[test]
    fn test_service_state_display() {
        assert_eq!(ServiceState::Running.to_string(), "Running");
        assert_eq!(ServiceState::Stopped.to_string(), "Stopped");
    }

    #[test]
    fn test_performance_tier_default() {
        assert_eq!(PerformanceTier::default(), PerformanceTier::Standard);
    }

    #[test]
    fn round5_performance_tier_display_impl() {
        assert_eq!(PerformanceTier::Ultra.to_string(), "Ultra");
        assert_eq!(PerformanceTier::Economy.to_string(), "Economy");
    }

    #[test]
    fn round5_performance_tier_serde_roundtrip() {
        for tier in [
            PerformanceTier::Ultra,
            PerformanceTier::High,
            PerformanceTier::Standard,
            PerformanceTier::Economy,
        ] {
            let json = serde_json::to_string(&tier).unwrap();
            let back: PerformanceTier = serde_json::from_str(&json).unwrap();
            assert_eq!(tier, back);
        }
    }

    #[test]
    fn round5_service_state_serde_roundtrip() {
        for state in [
            ServiceState::Running,
            ServiceState::Stopping,
            ServiceState::Error,
        ] {
            let json = serde_json::to_string(&state).unwrap();
            let back: ServiceState = serde_json::from_str(&json).unwrap();
            assert_eq!(state, back);
        }
    }

    #[test]
    fn round5_health_status_serde_roundtrip() {
        let h = HealthStatus::Unhealthy;
        let json = serde_json::to_string(&h).unwrap();
        let back: HealthStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(h, back);
    }
}
