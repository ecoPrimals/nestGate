//! **SERVICE & HEALTH TYPES**
//!
//! Service information, health status, and cluster monitoring.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Service Information (v2 specific)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    /// Service identifier
    pub service_id: String,
    /// Service name
    pub service_name: String,
    /// Service type
    pub service_type: String,
    /// Service endpoint
    pub endpoint: String,
    /// Service status
    pub status: ServiceStatus,
    /// Timestamp
    pub timestamp: std::time::SystemTime,
}

/// Service Status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceStatus {
    Online,
    Offline,
    Degraded,
    Maintenance,
}

/// Health Status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    /// Status
    pub status: ServiceStatus,
    /// Uptime
    pub uptime: std::time::Duration,
    /// Last check
    pub last_check: std::time::SystemTime,
    /// Details
    pub details: HashMap<String, String>,
}

/// Health Check Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckPayload {
    /// Check type
    pub check_type: HealthCheckType,
}

/// Health Check Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthCheckType {
    Shallow,
    Deep,
    Storage,
    Network,
}

/// Status Update Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusUpdatePayload {
    /// Service ID
    pub service_id: String,
    /// New status
    pub status: ServiceStatus,
}

/// Node Role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeRole {
    Primary,
    Secondary,
    Observer,
}

/// Cluster Health
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClusterHealth {
    Healthy,
    Degraded,
    Critical,
}
