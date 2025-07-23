//! Universal ZFS Types and Error Handling
//!
//! Common types used across all ZFS service implementations with comprehensive
//! error handling and structured data types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use std::time::SystemTime;
use thiserror::Error;

/// Universal ZFS Result type
pub type UniversalZfsResult<T> = Result<T, UniversalZfsError>;

/// Comprehensive error types for universal ZFS operations
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum UniversalZfsError {
    #[error("Service unavailable: {message}")]
    ServiceUnavailable { message: String },

    #[error("Operation timeout after {duration:?}: {operation}")]
    Timeout {
        operation: String,
        duration: Duration,
    },

    #[error("Configuration error: {message}")]
    Configuration { message: String },

    #[error("Backend error: {backend} - {message}")]
    Backend { backend: String, message: String },

    #[error("Resource not found: {resource_type} '{name}'")]
    NotFound { resource_type: String, name: String },

    #[error("Permission denied: {operation}")]
    PermissionDenied { operation: String },

    #[error("Invalid input: {field} - {message}")]
    InvalidInput { field: String, message: String },

    #[error("Network error: {message}")]
    Network { message: String },

    #[error("Internal error: {message}")]
    Internal { message: String },

    #[error("Circuit breaker open: {service}")]
    CircuitBreakerOpen { service: String },

    #[error("Rate limit exceeded: {limit} requests per {window:?}")]
    RateLimitExceeded { limit: u32, window: Duration },

    #[error("Validation failed: {errors:?}")]
    ValidationFailed { errors: Vec<String> },
}

impl UniversalZfsError {
    pub fn service_unavailable(message: impl Into<String>) -> Self {
        Self::ServiceUnavailable {
            message: message.into(),
        }
    }

    pub fn timeout(operation: impl Into<String>, duration: Duration) -> Self {
        Self::Timeout {
            operation: operation.into(),
            duration,
        }
    }

    pub fn configuration(message: impl Into<String>) -> Self {
        Self::Configuration {
            message: message.into(),
        }
    }

    pub fn backend(backend: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Backend {
            backend: backend.into(),
            message: message.into(),
        }
    }

    pub fn not_found(resource_type: impl Into<String>, name: impl Into<String>) -> Self {
        Self::NotFound {
            resource_type: resource_type.into(),
            name: name.into(),
        }
    }

    pub fn permission_denied(operation: impl Into<String>) -> Self {
        Self::PermissionDenied {
            operation: operation.into(),
        }
    }

    pub fn invalid_input(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::InvalidInput {
            field: field.into(),
            message: message.into(),
        }
    }

    pub fn network(message: impl Into<String>) -> Self {
        Self::Network {
            message: message.into(),
        }
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal {
            message: message.into(),
        }
    }

    /// Convert to HTTP status code
    pub fn to_http_status(&self) -> u16 {
        match self {
            Self::ServiceUnavailable { .. } => 503,
            Self::Timeout { .. } => 408,
            Self::Configuration { .. } => 400,
            Self::Backend { .. } => 502,
            Self::NotFound { .. } => 404,
            Self::PermissionDenied { .. } => 403,
            Self::InvalidInput { .. } => 400,
            Self::Network { .. } => 502,
            Self::Internal { .. } => 500,
            Self::CircuitBreakerOpen { .. } => 503,
            Self::RateLimitExceeded { .. } => 429,
            Self::ValidationFailed { .. } => 400,
        }
    }
}

/// Pool information with comprehensive metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolInfo {
    pub name: String,
    pub health: PoolHealth,
    pub state: PoolState,
    pub capacity: PoolCapacity,
    pub devices: Vec<String>,
    pub properties: HashMap<String, String>,
    pub created_at: SystemTime,
    pub last_scrub: Option<SystemTime>,
    pub scrub_status: ScrubStatus,
    pub errors: Vec<String>,
}

/// Pool health status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PoolHealth {
    Online,
    Degraded,
    Faulted,
    Offline,
    Unknown,
}

/// Pool state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PoolState {
    Active,
    Exported,
    Destroyed,
    Spare,
    L2Cache,
    Unavailable,
    Unknown,
}

/// Pool capacity information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolCapacity {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub utilization_percent: f64,
}

/// Pool scrub status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ScrubStatus {
    None,
    InProgress { percent_complete: f64 },
    Completed { errors_found: u32 },
    Cancelled,
    Failed { reason: String },
}

/// Dataset information with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetInfo {
    pub name: String,
    pub dataset_type: DatasetType,
    pub used_space: u64,
    pub available_space: u64,
    pub mount_point: Option<String>,
    pub properties: HashMap<String, String>,
    pub created_at: SystemTime,
    pub parent: Option<String>,
    pub children: Vec<String>,
}

/// Dataset type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DatasetType {
    Filesystem,
    Volume,
    Snapshot,
    Bookmark,
}

/// Snapshot information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotInfo {
    pub name: String,
    pub dataset: String,
    pub created_at: SystemTime,
    pub size_bytes: u64,
    pub properties: HashMap<String, String>,
    pub description: Option<String>,
}

/// Configuration types for creating resources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolConfig {
    pub name: String,
    pub devices: Vec<String>,
    pub raid_level: Option<String>,
    pub properties: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetConfig {
    pub name: String,
    pub parent: Option<String>,
    pub dataset_type: DatasetType,
    pub properties: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotConfig {
    pub name: String,
    pub dataset: String,
    pub description: Option<String>,
    pub properties: HashMap<String, String>,
}

/// Service health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub service_name: String,
    pub status: ServiceStatus,
    pub last_check: SystemTime,
    pub zfs_available: bool,
    pub pools_healthy: bool,
    pub datasets_healthy: bool,
    pub system_healthy: bool,
    pub checks: Vec<HealthCheck>,
    pub metrics: Option<ServiceMetrics>,
}

/// Service status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ServiceStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

/// Individual health check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    pub name: String,
    pub status: ServiceStatus,
    pub message: String,
    pub duration: Duration,
}

/// Service metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMetrics {
    pub service_name: String,
    pub timestamp: SystemTime,
    pub uptime: Duration,
    pub requests_total: u64,
    pub requests_successful: u64,
    pub requests_failed: u64,
    pub average_response_time: Duration,
    pub error_rate: f64,
    pub circuit_breaker_state: String,
    pub active_connections: u32,
    pub custom_metrics: HashMap<String, f64>,
}

impl Default for ServiceMetrics {
    fn default() -> Self {
        Self {
            service_name: "unknown".to_string(),
            timestamp: SystemTime::now(),
            uptime: Duration::from_secs(0),
            requests_total: 0,
            requests_successful: 0,
            requests_failed: 0,
            average_response_time: Duration::from_millis(0),
            error_rate: 0.0,
            circuit_breaker_state: "CLOSED".to_string(),
            active_connections: 0,
            custom_metrics: HashMap::new(),
        }
    }
}
