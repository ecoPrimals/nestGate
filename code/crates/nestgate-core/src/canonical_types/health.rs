// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **HEALTH TYPES** — System health monitoring

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// Health status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Status values for Health
pub enum HealthStatus {
    /// System is fully operational
    Healthy,
    /// System is operational but with reduced performance
    Degraded,
    /// System is not operational
    Unhealthy,
    /// Health status cannot be determined
    Unknown,
}

/// Health check result
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Healthcheck
pub struct HealthCheck {
    /// Component being checked
    pub component: String,
    /// Health status of the component
    pub status: HealthStatus,
    /// Optional message with additional details
    pub message: Option<String>,
    /// Timestamp of the health check
    pub timestamp: SystemTime,
    /// Response time in milliseconds
    pub response_time_ms: u64,
    /// Additional metadata about the health check
    pub metadata: HashMap<String, serde_json::Value>,
}

/// System health summary
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Systemhealth
pub struct SystemHealth {
    /// Overall system health status
    pub overall_status: HealthStatus,
    /// Individual component health checks
    pub components: Vec<HealthCheck>,
    /// Timestamp of last health check update
    pub last_updated: SystemTime,
    /// System uptime in seconds
    pub uptime_seconds: u64,
}
