// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

use std::collections::HashMap;
///
/// Unified health status definitions for consistent health monitoring
/// across all NestGate services and components.
use serde::{Deserialize, Serialize};
/// Unified health status enumeration for all services
/// Provides consistent health reporting across the ecosystem
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Status values for UnifiedHealth
pub enum UnifiedHealthStatus {
    /// Service is healthy and operating normally
    Healthy,
    /// Service is degraded but still functional
    Degraded,
    /// Service is unhealthy and may not be functioning
    Unhealthy,
    /// Service health status is unknown
    Unknown,
    /// Service is starting up
    Starting,
    /// Service is shutting down
    Stopping,
    /// Service is offline
    Offline,
    /// Service is in maintenance mode
    Maintenance,
    /// Service has encountered an error
    Error,
    /// Service is in warning state (still functional but needs attention)
    Warning,
    /// Service is in critical state (major issues)
    Critical,
    /// Custom health status with description
    Custom(String),
}
impl Default for UnifiedHealthStatus {
    /// Returns the default instance
    fn default() -> Self {
        Self::Unknown
    }
}

impl UnifiedHealthStatus {
    /// Check if the status indicates the component is operational
    pub fn is_operational(&self) -> bool {
        matches!(self, Self::Healthy | Self::Warning | Self::Degraded)
    }

    /// Check if the status requires immediate attention
    pub fn requires_attention(&self) -> bool {
        matches!(self, Self::Unhealthy | Self::Critical)
    }

    /// Get the severity level (0 = healthy, 11 = critical)
    pub fn severity(&self) -> u8 {
        match self {
            Self::Healthy => 0,
            Self::Warning => 1,
            Self::Degraded => 2,
            Self::Unhealthy => 3,
            Self::Critical => 4,
            Self::Unknown => 5,
            Self::Error => 6,
            Self::Maintenance => 7,
            Self::Starting => 8,
            Self::Stopping => 9,
            Self::Offline => 10,
            Self::Custom(_) => 11,
        }
    }

    /// Get priority level for health status (lower = higher priority)
    pub fn priority(&self) -> u8 {
        match self {
            Self::Healthy => 0,
            Self::Warning => 1,
            Self::Degraded => 2,
            Self::Maintenance => 3,
            Self::Critical => 4,
            Self::Unknown => 5,
            Self::Unhealthy => 6,
            Self::Error => 7,
            Self::Starting => 8,
            Self::Stopping => 9,
            Self::Offline => 10,
            Self::Custom(_) => 11,
        }
    }
}

/// Comprehensive health report for a component
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Healthreport
pub struct HealthReport {
    /// Overall health status
    pub status: UnifiedHealthStatus,
    /// Component identifier
    pub component_id: String,
    /// Detailed status message
    pub message: String,
    /// Health check timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Additional health metrics
    pub metrics: HashMap<String, f64>,
    /// Any health-related warnings
    pub warnings: Vec<String>,
    /// Any health-related errors
    pub errors: Vec<String>,
}
impl Default for HealthReport {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            status: UnifiedHealthStatus::Unknown,
            component_id: "unknown".to_string(),
            message: "No health data available".to_string(),
            timestamp: chrono::Utc::now(),
            metrics: HashMap::new(),
            warnings: Vec::new(),
            errors: Vec::new(),
        }
    }
}

impl HealthReport {
    /// Create a healthy report
    #[must_use]
    pub fn healthy(component_id: String) -> Self {
        Self {
            status: UnifiedHealthStatus::Healthy,
            component_id,
            message: "Component is healthy".to_string(),
            timestamp: chrono::Utc::now(),
            metrics: HashMap::new(),
            warnings: Vec::new(),
            errors: Vec::new(),
        }
    }

    /// Create a warning report
    #[must_use]
    pub fn warning(component_id: String, message: String) -> Self {
        Self {
            status: UnifiedHealthStatus::Warning,
            component_id,
            message,
            timestamp: chrono::Utc::now(),
            metrics: HashMap::new(),
            warnings: Vec::new(),
            errors: Vec::new(),
        }
    }

    /// Create a critical report
    #[must_use]
    pub fn critical(component_id: String, message: String) -> Self {
        Self {
            status: UnifiedHealthStatus::Critical,
            component_id,
            message,
            timestamp: chrono::Utc::now(),
            metrics: HashMap::new(),
            warnings: Vec::new(),
            errors: Vec::new(),
        }
    }
}
