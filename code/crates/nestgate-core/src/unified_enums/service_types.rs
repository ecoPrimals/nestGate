/// Service Classification Enums
/// This module contains enums related to service types, health status,
/// and service lifecycle management.
use serde::{Deserialize, Serialize};
use std::fmt;
// ==================== SECTION ====================

/// **THE** `ServiceType` - unified across all modules
/// Replaces 2+ fragmented `ServiceType` definitions
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Types of UnifiedService
pub enum UnifiedServiceType {
    /// AI and machine learning services
    AI,
    /// Storage and data management services
    Storage,
    /// Orchestration and coordination services
    Orchestration,
    /// Security and authentication services
    Security,
    /// Compute and processing services
    Compute,
    /// Network and communication services
    Network,
    /// Monitoring and observability services
    Monitoring,
    /// Universal adapter for ecosystem integration
    Adapter,
    /// API gateway and routing services
    Gateway,
    /// Background worker services
    Worker,
    /// General purpose or unspecified service
    Generic,
    /// Unknown or unclassified service
    Unknown,
    /// Custom service type with name
    Custom(String),
}
impl Default for UnifiedServiceType {
    /// Returns the default instance
    fn default() -> Self {
        Self::Unknown
    }
}

impl fmt::Display for UnifiedServiceType {
    /// Fmt
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AI => write!(f, "ai"),
            Self::Storage => write!(f, "storage"),
            Self::Orchestration => write!(f, "orchestration"),
            Self::Security => write!(f, "security"),
            Self::Compute => write!(f, "compute"),
            Self::Network => write!(f, "network"),
            Self::Monitoring => write!(f, "monitoring"),
            Self::Adapter => write!(f, "adapter"),
            Self::Gateway => write!(f, "gateway"),
            Self::Worker => write!(f, "worker"),
            Self::Generic => write!(f, "generic"),
            Self::Unknown => write!(f, "unknown"),
            Self::Custom(name) => write!(f, "{name}"),
        }
    }
}

// ==================== SECTION ====================

/// **THE** `HealthStatus` - unified across all modules
/// Replaces 4+ fragmented `HealthStatus` definitions across health checks, monitoring, etc.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Status values for UnifiedHealth
pub enum UnifiedHealthStatus {
    /// Service is healthy and operational
    Healthy,
    /// Service is degraded but functional
    Degraded,
    /// Service is unhealthy but attempting recovery
    Unhealthy,
    /// Service is completely offline
    Offline,
    /// Service is starting up
    Starting,
    /// Service is shutting down
    Stopping,
    /// Service is in maintenance mode
    Maintenance,
    /// Health status unknown
    Unknown,
    /// Warning state - service has issues but is operational
    Warning,
    /// Critical state - service has severe issues
    Critical,
    /// Error state - service encountered errors
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

impl fmt::Display for UnifiedHealthStatus {
    /// Fmt
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Healthy => write!(f, "healthy"),
            Self::Degraded => write!(f, "degraded"),
            Self::Unhealthy => write!(f, "unhealthy"),
            Self::Offline => write!(f, "offline"),
            Self::Starting => write!(f, "starting"),
            Self::Stopping => write!(f, "stopping"),
            Self::Maintenance => write!(f, "maintenance"),
            Self::Unknown => write!(f, "unknown"),
            Self::Warning => write!(f, "warning"),
            Self::Critical => write!(f, "critical"),
            Self::Error => write!(f, "error"),
            Self::Custom(status) => write!(f, "{status}"),
        }
    }
}

// ==================== SECTION ====================

/// **THE** `ServiceState` - unified across all modules  
/// Replaces 6+ fragmented `ServiceStatus` enum definitions across diagnostics, network, etc.
/// Note: This is different from `UnifiedServiceStatus` struct which contains metadata
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Unifiedservicestate
pub enum UnifiedServiceState {
    /// Service is running and operational
    Running,
    /// Service is stopped
    Stopped,
    /// Service is starting up
    Starting,
    /// Service is shutting down
    Stopping,
    /// Service is in error state
    Error,
    /// Service is paused/suspended
    Paused,
    /// Service is in maintenance mode
    Maintenance,
    /// Service state is unknown
    Unknown,
    /// Custom service state
    Custom(String),
}
impl Default for UnifiedServiceState {
    /// Returns the default instance
    fn default() -> Self {
        Self::Unknown
    }
}

impl fmt::Display for UnifiedServiceState {
    /// Fmt
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Running => write!(f, "running"),
            Self::Stopped => write!(f, "stopped"),
            Self::Starting => write!(f, "starting"),
            Self::Stopping => write!(f, "stopping"),
            Self::Error => write!(f, "error"),
            Self::Paused => write!(f, "paused"),
            Self::Maintenance => write!(f, "maintenance"),
            Self::Unknown => write!(f, "unknown"),
            Self::Custom(state) => write!(f, "{state}"),
        }
    }
}

// ==================== SECTION ====================

/// **THE** `ConnectionStatus` - unified across all modules
/// Replaces `ConnectionStatus` definitions across network and service modules
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Status values for UnifiedConnection
pub enum UnifiedConnectionStatus {
    /// Connection is active and operational
    Connected,
    /// Connection is being established
    Connecting,
    /// Connection is closed
    Disconnected,
    /// Connection failed to establish
    Failed,
    /// Connection is being retried
    Retrying,
    /// Connection timed out
    Timeout,
    /// Connection was refused
    Refused,
    /// Connection status is unknown
    Unknown,
    /// Custom connection status
    Custom(String),
}
impl Default for UnifiedConnectionStatus {
    /// Returns the default instance
    fn default() -> Self {
        Self::Unknown
    }
}

impl fmt::Display for UnifiedConnectionStatus {
    /// Fmt
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Connected => write!(f, "connected"),
            Self::Connecting => write!(f, "connecting"),
            Self::Disconnected => write!(f, "disconnected"),
            Self::Failed => write!(f, "failed"),
            Self::Retrying => write!(f, "retrying"),
            Self::Timeout => write!(f, "timeout"),
            Self::Refused => write!(f, "refused"),
            Self::Unknown => write!(f, "unknown"),
            Self::Custom(status) => write!(f, "{status}"),
        }
    }
}

// Implement UnifiedEnum trait for UnifiedServiceType
impl super::UnifiedEnum for UnifiedServiceType {
    /// Returns as Str
    fn as_str(&self) -> &str {
        match self {
            Self::AI => "ai",
            Self::Storage => "storage",
            Self::Orchestration => "orchestration",
            Self::Security => "security",
            Self::Compute => "compute",
            Self::Network => "network",
            Self::Monitoring => "monitoring",
            Self::Adapter => "adapter",
            Self::Gateway => "gateway",
            Self::Worker => "worker",
            Self::Generic => "generic",
            Self::Unknown => "unknown",
            Self::Custom(name) => name,
        }
    }

    /// Creates from Str
    fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "ai" => Self::AI,
            "storage" => Self::Storage,
            "orchestration" => Self::Orchestration,
            "security" => Self::Security,
            "compute" => Self::Compute,
            "network" => Self::Network,
            "monitoring" => Self::Monitoring,
            "adapter" => Self::Adapter,
            "gateway" => Self::Gateway,
            "worker" => Self::Worker,
            "generic" => Self::Generic,
            "unknown" => Self::Unknown,
            custom => Self::Custom(custom.to_string()),
        }
    }

    /// Checks if Custom
    fn is_custom(&self) -> bool {
        matches!(self, Self::Custom(_))
    }
}
