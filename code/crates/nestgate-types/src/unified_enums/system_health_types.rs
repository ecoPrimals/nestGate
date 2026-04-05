// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

/// System Health and Test Classification Enums
/// This module contains enums related to system health monitoring,
/// test classifications, and operational status.
use serde::{Deserialize, Serialize};
use std::fmt;
// ==================== SECTION ====================

/// **THE** `SystemStatus` - unified across all modules
/// Replaces `SystemStatus` definitions across system monitoring modules
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Status values for `UnifiedSystem`
pub enum UnifiedSystemStatus {
    /// System is operational
    Operational,
    /// System is starting up
    Starting,
    /// System is shutting down
    Stopping,
    /// System is in maintenance mode
    Maintenance,
    /// System has errors but is running
    Degraded,
    /// System is completely down
    Down,
    /// System status is unknown
    Unknown,
    /// Custom system status
    Custom(String),
}
impl Default for UnifiedSystemStatus {
    /// Returns the default instance
    fn default() -> Self {
        Self::Unknown
    }
}

impl fmt::Display for UnifiedSystemStatus {
    /// Fmt
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Operational => write!(f, "operational"),
            Self::Starting => write!(f, "starting"),
            Self::Stopping => write!(f, "stopping"),
            Self::Maintenance => write!(f, "maintenance"),
            Self::Degraded => write!(f, "degraded"),
            Self::Down => write!(f, "down"),
            Self::Unknown => write!(f, "unknown"),
            Self::Custom(status) => write!(f, "{status}"),
        }
    }
}

// ==================== SECTION ====================

/// **THE** `TestType` - unified across all modules
/// Replaces `TestType` definitions in test frameworks and automation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Types of `UnifiedTest`
pub enum UnifiedTestType {
    /// Unit test
    Unit,
    /// Integration test
    Integration,
    /// End-to-end test
    EndToEnd,
    /// Performance test
    Performance,
    /// Load test
    Load,
    /// Stress test
    Stress,
    /// Security test
    Security,
    /// Chaos test
    Chaos,
    /// Functional test
    Functional,
    /// Regression test
    Regression,
    /// Smoke test
    Smoke,
    /// Custom test type
    Custom(String),
}
impl Default for UnifiedTestType {
    /// Returns the default instance
    fn default() -> Self {
        Self::Unit
    }
}

impl fmt::Display for UnifiedTestType {
    /// Fmt
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unit => write!(f, "unit"),
            Self::Integration => write!(f, "integration"),
            Self::EndToEnd => write!(f, "end_to_end"),
            Self::Performance => write!(f, "performance"),
            Self::Load => write!(f, "load"),
            Self::Stress => write!(f, "stress"),
            Self::Security => write!(f, "security"),
            Self::Chaos => write!(f, "chaos"),
            Self::Functional => write!(f, "functional"),
            Self::Regression => write!(f, "regression"),
            Self::Smoke => write!(f, "smoke"),
            Self::Custom(test_type) => write!(f, "{test_type}"),
        }
    }
}

// ==================== SECTION ====================

/// **THE** `MonitoringStatus` - unified across all modules
/// Replaces `MonitoringStatus` definitions in observability modules
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Status values for `UnifiedMonitoring`
pub enum UnifiedMonitoringStatus {
    /// Monitoring is active
    Active,
    /// Monitoring is paused
    Paused,
    /// Monitoring is disabled
    Disabled,
    /// Monitoring configuration error
    ConfigError,
    /// Monitoring connection error
    ConnectionError,
    /// Custom monitoring status
    Custom(String),
}
impl Default for UnifiedMonitoringStatus {
    /// Returns the default instance
    fn default() -> Self {
        Self::Active
    }
}

impl fmt::Display for UnifiedMonitoringStatus {
    /// Fmt
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Paused => write!(f, "paused"),
            Self::Disabled => write!(f, "disabled"),
            Self::ConfigError => write!(f, "config_error"),
            Self::ConnectionError => write!(f, "connection_error"),
            Self::Custom(status) => write!(f, "{status}"),
        }
    }
}
