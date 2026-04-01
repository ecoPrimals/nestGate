// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **SERVICE TYPES** — Core service identification and management

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

use crate::constants::system::DEFAULT_INSTANCE_NAME;

/// Service identification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Serviceid
pub struct ServiceId(pub String);

impl Default for ServiceId {
    /// Returns the default instance
    fn default() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}

/// Service states
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Servicestate
pub enum ServiceState {
    /// Service is in the process of starting up
    Starting,
    /// Service is running and operational
    Running,
    /// Service is in the process of shutting down
    Stopping,
    /// Service has stopped and is not running
    Stopped,
    /// Service has encountered a failure and cannot operate
    Failed,
    /// Service is under maintenance and temporarily unavailable
    Maintenance,
    /// Service state is unknown or cannot be determined
    Unknown,
}

/// Service types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Types of Service
pub enum ServiceType {
    /// API service handling HTTP/REST requests
    Api,
    /// Storage service managing data persistence
    Storage,
    /// Network service handling connectivity and routing
    Network,
    /// Security service managing authentication and authorization
    Security,
    /// Monitoring service collecting metrics and health data
    Monitoring,
    /// Automation service handling scheduled tasks and workflows
    Automation,
    /// MCP (Model Context Protocol) service
    Mcp,
    /// ZFS storage management service
    Zfs,
    /// Custom service type with user-defined name
    Custom(String),
    /// Generic service type for unspecified services
    Generic,
    /// Compute service handling processing workloads
    Compute,
}

/// Service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Service
pub struct ServiceConfig {
    /// Unique service identifier
    pub id: ServiceId,
    /// Human-readable service name
    pub name: String,
    /// Type of service
    pub service_type: ServiceType,
    /// Current operational state
    pub state: ServiceState,
    /// Network port if applicable
    pub port: Option<u16>,
    /// Host address if applicable
    pub host: Option<String>,
    /// Additional service metadata
    pub metadata: HashMap<String, String>,
    /// Timestamp when service was created
    pub created_at: SystemTime,
    /// Timestamp of last update
    pub updated_at: SystemTime,
}
/// **CANONICAL SERVICE INFO** - Consolidates all `ServiceInfo` definitions
///
/// This replaces duplicate `ServiceInfo` structs from:
/// - diagnostics/types.rs
/// - automation/src/types/ecosystem.rs
/// - diagnostics/metrics.rs
/// - And other scattered definitions
///
/// Service Information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    // Core identification
    /// Unique service identifier
    pub service_id: String,
    /// Primary service name
    pub service_name: String,
    /// Service name (backward compatibility alias)
    pub name: String,
    /// Service version string
    pub version: String,

    // Status and health
    /// Current operational status
    pub status: ServiceState,
    /// Human-readable health status
    pub health_status: String,
    /// Health status (backward compatibility)
    pub health: Option<String>,

    // Runtime information
    /// Service uptime in seconds
    pub uptime_seconds: Option<u64>,
    /// Process ID if available
    pub pid: Option<u32>,
    /// Service start timestamp
    pub start_time: Option<SystemTime>,

    // Performance metrics
    /// CPU usage percentage
    pub cpu_percent: Option<f64>,
    /// Memory usage in bytes
    pub memory_bytes: Option<u64>,

    // Configuration and metadata
    /// Service capabilities list
    pub capabilities: Vec<String>,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
    /// Optional service description
    pub description: Option<String>,
}

impl Default for ServiceInfo {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            service_id: ServiceId::default().0,
            service_name: DEFAULT_INSTANCE_NAME.to_string(),
            name: DEFAULT_INSTANCE_NAME.to_string(),
            version: "1.0.0".to_string(),
            status: ServiceState::Unknown,
            health_status: "unknown".to_string(),
            health: None,
            uptime_seconds: None,
            pid: None,
            start_time: None,
            cpu_percent: None,
            memory_bytes: None,
            capabilities: Vec::new(),
            metadata: HashMap::new(),
            description: None,
        }
    }
}

/// Service metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Servicemetrics
pub struct ServiceMetrics {
    /// CPU usage as a percentage (0-100)
    pub cpu_usage_percent: f64,
    /// Memory usage in bytes
    pub memory_usage_bytes: u64,
    /// Service uptime in seconds
    pub uptime_seconds: u64,
    /// Average requests per second
    pub requests_per_second: f64,
    /// Error rate as a percentage (0-100)
    pub error_rate_percent: f64,
    /// Timestamp of last metrics update
    pub last_updated: SystemTime,
}

/// Service metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Servicemetadata
pub struct ServiceMetadata {
    /// Unique service identifier
    pub service_id: String,
    /// Type of service
    pub service_type: ServiceType,
    /// Timestamp when service was created
    pub created_at: SystemTime,
    /// Current health/operational status
    pub health_status: ServiceState,
    /// List of service capabilities
    pub capabilities: Vec<String>,
    /// Service endpoint URLs
    pub endpoints: HashMap<String, String>,
    /// Service configuration parameters
    pub configuration: HashMap<String, String>,
}
