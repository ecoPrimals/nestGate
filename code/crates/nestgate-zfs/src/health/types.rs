// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Health status, alerts, and shared type aliases for the ZFS health subsystem.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

/// Health status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Status values for Health
pub enum HealthStatus {
    /// Healthy
    Healthy,
    /// Warning
    Warning,
    /// Critical
    Critical,
    /// Unknown
    Unknown,
}

/// Health report for a component
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Healthreport
pub struct HealthReport {
    /// Component Type
    pub component_type: String,
    /// Component name
    pub component_name: String,
    /// Status
    pub status: HealthStatus,
    /// Last Check
    pub last_check: SystemTime,
    /// Details
    pub details: String,
}

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Alertlevel
pub enum AlertLevel {
    /// Info
    Info,
    /// Warning
    Warning,
    /// Critical
    Critical,
}

/// Alert information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Alert
pub struct Alert {
    /// Unique identifier
    pub id: String,
    /// Level
    pub level: AlertLevel,
    /// Message
    pub message: String,
    /// Timestamp
    pub timestamp: SystemTime,
    /// Component
    pub component: String,
}

/// Type alias for health data storage
pub type HealthDataMap = Arc<tokio::sync::RwLock<HashMap<String, HealthReport>>>;
/// Type alias for monitoring task handles
pub type MonitoringTasks = Option<(tokio::task::JoinHandle<()>, tokio::task::JoinHandle<()>)>;
/// Type alias for health status storage
pub type HealthStatusMap = Arc<tokio::sync::RwLock<HashMap<String, HealthStatus>>>;
/// Type alias for background task storage
pub type BackgroundTasks = Arc<tokio::sync::RwLock<Vec<tokio::task::JoinHandle<()>>>>;

impl HealthStatus {
    /// Returns `true` if the health status is critical.
    #[must_use]
    pub const fn is_critical(&self) -> bool {
        matches!(self, Self::Critical)
    }

    /// Returns `true` if the health status is healthy.
    #[must_use]
    pub const fn is_healthy(&self) -> bool {
        matches!(self, Self::Healthy)
    }
}

impl std::fmt::Display for HealthStatus {
    /// Fmt
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Healthy => write!(f, "Healthy"),
            Self::Warning => write!(f, "Warning"),
            Self::Critical => write!(f, "Critical"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}
