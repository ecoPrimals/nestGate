/// Health Status Module
/// Provides standardized health status definitions and utilities
/// **CONSOLIDATION**: Unifies health status patterns across the system
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Universal health status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HealthStatus {
    /// Service is fully operational
    Healthy,
    /// Service has minor issues but is functional
    Degraded,
    /// Service is experiencing significant issues
    Unhealthy,
    /// Service status cannot be determined
    Unknown,
}

impl Default for HealthStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

impl HealthStatus {
    /// Check if the status represents a healthy state
    pub fn is_healthy(&self) -> bool {
        matches!(self, HealthStatus::Healthy)
    }

    /// Check if the status represents an unhealthy state
    pub fn is_unhealthy(&self) -> bool {
        matches!(self, HealthStatus::Unhealthy)
    }

    /// Get a human-readable description of the status
    pub fn description(&self) -> &'static str {
        match self {
            HealthStatus::Healthy => "Service is operating normally",
            HealthStatus::Degraded => "Service has minor issues but is functional",
            HealthStatus::Unhealthy => "Service is experiencing significant problems",
            HealthStatus::Unknown => "Service status cannot be determined",
        }
    }
}

/// Health check result with detailed information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResult {
    /// Overall health status
    pub status: HealthStatus,
    /// Check timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Detailed health information
    pub details: HashMap<String, serde_json::Value>,
    /// Optional error message
    pub error: Option<String>,
}

/// Unified health status type alias for compatibility
pub type UnifiedHealthStatus = HealthStatus;

/// Health state type alias for compatibility
pub type HealthState = HealthStatus;

/// Service metrics for health monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedServiceMetrics {
    /// CPU usage percentage
    pub cpu_usage: f64,
    /// Memory usage in bytes
    pub memory_usage: u64,
    /// Network I/O metrics
    pub network_io: HashMap<String, u64>,
    /// Disk I/O metrics
    pub disk_io: HashMap<String, u64>,
    /// Custom metrics
    pub custom: HashMap<String, serde_json::Value>,
}
