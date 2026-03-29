// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//
// **CANONICAL MODERNIZATION**: Consolidated enum definitions that replace
// the deprecated unified_enums module.

use serde::{Deserialize, Serialize};

/// **UNIFIED CAPABILITY TYPE**
///
/// Represents different types of service capabilities in the `NestGate` ecosystem
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Types of `UnifiedCapability`
pub enum UnifiedCapabilityType {
    /// Storage capabilities (ZFS, NAS, etc.)
    Storage,
    /// Network capabilities (routing, protocols, etc.)
    Network,
    /// Security capabilities (auth, encryption, etc.)
    Security,
    /// Monitoring capabilities (metrics, health, etc.)
    Monitoring,
    /// AI capabilities (automation, intelligence, etc.)
    AI,
    /// Orchestration capabilities (service management, etc.)
    Orchestration,
    /// Compute capabilities (processing, etc.)
    Compute,
    /// Generic/other capabilities
    Generic,
    /// Custom capability type
    Custom(String),
}
impl Default for UnifiedCapabilityType {
    /// Returns the default instance
    fn default() -> Self {
        Self::Generic
    }
}

/// **UNIFIED SERVICE TYPE**
///
/// Represents different types of services in the `NestGate` ecosystem
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Types of `UnifiedService`
pub enum UnifiedServiceType {
    /// Storage services
    Storage,
    /// Network services
    Network,
    /// Security services
    Security,
    /// Monitoring services
    Monitoring,
    /// AI services
    AI,
    /// Orchestration services
    Orchestration,
    /// Compute services
    Compute,
    /// Generic services
    Generic,
    /// Custom service type
    Custom(String),
}
impl Default for UnifiedServiceType {
    /// Returns the default instance
    fn default() -> Self {
        Self::Generic
    }
}

/// **UNIFIED SERVICE STATE**
///
/// Represents the current state of a service
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Unifiedservicestate
pub enum UnifiedServiceState {
    /// Service is running normally
    Running,
    /// Service is stopped
    Stopped,
    /// Service is starting up
    Starting,
    /// Service is shutting down
    Stopping,
    /// Service is paused
    Paused,
    /// Service is in maintenance mode
    Maintenance,
    /// Service state is unknown
    Unknown,
    /// Service has an error
    Error,
    /// Custom service state
    Custom(String),
}
impl Default for UnifiedServiceState {
    /// Returns the default instance
    fn default() -> Self {
        Self::Unknown
    }
}

/// **UNIFIED HEALTH STATUS**
///
/// Represents the health status of a service or component
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Status values for `UnifiedHealth`
pub enum UnifiedHealthStatus {
    /// Component is healthy
    Healthy,
    /// Component is degraded but functional
    Degraded,
    /// Component is unhealthy
    Unhealthy,
    /// Component is offline
    Offline,
    /// Component is starting
    Starting,
    /// Component is stopping
    Stopping,
    /// Component is in maintenance
    Maintenance,
    /// Component status is unknown
    Unknown,
    /// Component is in critical state
    Critical,
    /// Component has warnings
    Warning,
    /// Component has errors
    Error,
    /// Custom health status
    Custom(String),
}
impl Default for UnifiedHealthStatus {
    /// Returns the default instance
    fn default() -> Self {
        Self::Unknown
    }
}

/// **UNIFIED TIER TYPE**
///
/// Represents different storage tiers
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Types of `UnifiedTier`
pub enum UnifiedTierType {
    /// Hot tier (frequently accessed)
    Hot,
    /// Warm tier (occasionally accessed)
    Warm,
    /// Cold tier (rarely accessed)
    Cold,
    /// Archive tier (long-term storage)
    Archive,
    /// Custom tier
    Custom(String),
}
impl Default for UnifiedTierType {
    /// Returns the default instance
    fn default() -> Self {
        Self::Hot
    }
}

/// Unified file type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Types of `UnifiedFile`
pub enum UnifiedFileType {
    /// Document file
    Document,
    /// Image file
    Image,
    /// Video file
    Video,
    /// Audio file
    Audio,
    /// Archive file
    Archive,
    /// Database file
    Database,
    /// Configuration file
    Configuration,
    /// Log file
    Log,
    /// Binary executable
    Binary,
    /// Source code file
    SourceCode,
    /// Unknown file type
    Unknown,
}
impl Default for UnifiedFileType {
    /// Returns the default instance
    fn default() -> Self {
        Self::Unknown
    }
}
