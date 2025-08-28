// Universal ZFS Types and Error Handling
//
// Common types used across all ZFS service implementations with comprehensive
// error handling and structured data types.

use nestgate_core::error::domain_errors::RateLimitInfo;
use nestgate_core::error::{IdioResult, NestGateError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use std::time::SystemTime;
use thiserror::Error;

// ==================== SECTION ====================

/// **CANONICAL**: Universal ZFS Result type using IdioResult
/// This follows the canonical Result<T,E> pattern with domain-specific error type
pub type UniversalZfsResult<T> = IdioResult<T, UniversalZfsError>;

// DEPRECATED TYPE REMOVED: LegacyZfsResult<T> - Use UniversalZfsResult<T> instead

/// Comprehensive error types for universal ZFS operations
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum UniversalZfsError {
    #[error("Service unavailable: {message}")]
    /// Service is temporarily unavailable
    ServiceUnavailable {
        /// Error message describing the unavailability
        message: String,
    },

    #[error("Operation timeout after {duration:?}: {operation}")]
    /// Operation timed out
    Timeout {
        /// Operation that timed out
        operation: String,
        /// Duration after which timeout occurred
        duration: Duration,
    },

    #[error("Configuration error: {message}")]
    /// Configuration-related error
    Configuration {
        /// Configuration error message
        message: String,
    },

    #[error("Backend error: {backend} - {message}")]
    /// Backend system error
    Backend {
        /// Name of the backend that failed
        backend: String,
        /// Backend error message
        message: String,
    },

    #[error("Resource not found: {resource_type} '{name}'")]
    /// Requested resource was not found
    NotFound {
        /// Type of resource that was not found
        resource_type: String,
        /// Name of the resource that was not found
        name: String,
    },

    #[error("Permission denied: {operation}")]
    /// Permission denied for the requested operation
    PermissionDenied {
        /// Operation that was denied
        operation: String,
    },

    #[error("Invalid input: {field} - {message}")]
    /// Invalid input provided
    InvalidInput {
        /// Field that contains invalid input
        field: String,
        /// Description of the invalid input
        message: String,
    },

    #[error("Network error: {message}")]
    /// Network communication error
    Network {
        /// Network error message
        message: String,
    },

    #[error("Internal error: {message}")]
    /// Internal system error
    Internal {
        /// Error message
        message: String,
    },

    #[error("Circuit breaker open: {service}")]
    /// Circuit breaker is open for a service
    CircuitBreakerOpen {
        /// Name of the affected service
        service: String,
    },

    #[error("Rate limit exceeded: {limit} requests per {window:?}")]
    /// Rate limit has been exceeded
    RateLimitExceeded {
        /// Maximum allowed requests
        limit: u32,
        /// Time window for the limit
        window: Duration,
    },

    #[error("Validation failed: {errors:?}")]
    /// Input validation failed
    ValidationFailed {
        /// List of validation errors
        errors: Vec<String>,
    },

    #[error("Command failed: {command} - {message}")]
    /// ZFS command execution failed
    CommandFailed {
        /// Command that failed
        command: String,
        /// Error message from command execution
        message: String,
    },
}

// ==================== SECTION ====================

impl From<UniversalZfsError> for NestGateError {
    fn from(err: UniversalZfsError) -> Self {
        match err {
            UniversalZfsError::ServiceUnavailable { message } => {
                NestGateError::service_unavailable("universal_zfs".to_string(), message)
            }
            UniversalZfsError::Timeout {
                operation,
                duration,
            } => NestGateError::Timeout {
                operation,
                duration,
                retryable: true,
                context: None,
                suggested_timeout: Some(duration * 2),
            },
            UniversalZfsError::Configuration { message } => {
                NestGateError::configuration_error(message, Some("zfs".to_string()))
            }
            UniversalZfsError::Backend { backend, message } => {
                NestGateError::UniversalZfs(Box::new(nestgate_core::error::UniversalZfsErrorData {
                    message,
                    operation: "backend_operation".to_string(),
                    backend: Some(backend),
                    resource: None,
                    timeout_duration: None,
                    circuit_breaker_open: false,
                    rate_limit_info: None,
                }))
            }
            UniversalZfsError::NotFound {
                resource_type,
                name,
            } => {
                NestGateError::UniversalZfs(Box::new(nestgate_core::error::UniversalZfsErrorData {
                    message: format!("{} '{}' not found", resource_type, name),
                    operation: "resource_lookup".to_string(),
                    backend: None,
                    resource: Some(name),
                    timeout_duration: None,
                    circuit_breaker_open: false,
                    rate_limit_info: None,
                }))
            }
            UniversalZfsError::PermissionDenied { operation } => {
                NestGateError::permission_denied_error("zfs_resource".to_string(), operation)
            }
            UniversalZfsError::InvalidInput { field, message } => {
                NestGateError::invalid_input(field, message)
            }
            UniversalZfsError::Network { message } => {
                NestGateError::Network(Box::new(nestgate_core::error::NetworkErrorData {
                    message,
                    operation: "zfs_network".to_string(),
                    endpoint: None,
                    context: None,
                }))
            }
            UniversalZfsError::Internal { message } => NestGateError::Internal {
                message,
                location: Some("universal_zfs".to_string()),
                context: None,
                is_bug: false,
            },
            UniversalZfsError::CircuitBreakerOpen { service } => {
                NestGateError::UniversalZfs(Box::new(nestgate_core::error::UniversalZfsErrorData {
                    message: format!("Circuit breaker open for service: {}", service),
                    operation: "service_access".to_string(),
                    backend: Some(service),
                    resource: None,
                    timeout_duration: None,
                    circuit_breaker_open: true,
                    rate_limit_info: None,
                }))
            }
            UniversalZfsError::RateLimitExceeded { limit, window } => {
                NestGateError::UniversalZfs(Box::new(nestgate_core::error::UniversalZfsErrorData {
                    message: format!("Rate limit exceeded: {} per {:?}", limit, window),
                    operation: "rate_limited_operation".to_string(),
                    backend: None,
                    resource: None,
                    timeout_duration: None,
                    circuit_breaker_open: false,
                    rate_limit_info: Some(RateLimitInfo {
                        limit,
                        window,
                        current_usage: limit,
                        reset_time: std::time::SystemTime::now() + window,
                    }),
                }))
            }
            UniversalZfsError::ValidationFailed { errors } => NestGateError::Validation {
                field: Some("multiple_fields".to_string()),
                message: errors.join("; "),
                current_value: None,
                expected: Some("valid input".to_string()),
                user_error: true,
                context: None,
            },
            UniversalZfsError::CommandFailed { command, message } => {
                NestGateError::UniversalZfs(Box::new(nestgate_core::error::UniversalZfsErrorData {
                    message,
                    operation: "command_execution".to_string(),
                    backend: None,
                    resource: Some(command),
                    timeout_duration: None,
                    circuit_breaker_open: false,
                    rate_limit_info: None,
                }))
            }
        }
    }
}

impl UniversalZfsError {
    /// Create a service unavailable error
    pub fn service_unavailable(message: impl Into<String>) -> Self {
        Self::ServiceUnavailable {
            message: message.into(),
        }
    }

    /// Create a timeout error
    pub fn timeout(operation: impl Into<String>, duration: Duration) -> Self {
        Self::Timeout {
            operation: operation.into(),
            duration,
        }
    }

    /// Create a configuration error
    pub fn configuration(message: impl Into<String>) -> Self {
        Self::Configuration {
            message: message.into(),
        }
    }

    /// Create a backend error
    pub fn backend(backend: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Backend {
            backend: backend.into(),
            message: message.into(),
        }
    }

    /// Create a not found error
    pub fn not_found(resource_type: impl Into<String>, name: impl Into<String>) -> Self {
        Self::NotFound {
            resource_type: resource_type.into(),
            name: name.into(),
        }
    }

    /// Create a permission denied error
    pub fn permission_denied(operation: impl Into<String>) -> Self {
        Self::PermissionDenied {
            operation: operation.into(),
        }
    }

    /// Create an invalid input error
    pub fn invalid_input(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::InvalidInput {
            field: field.into(),
            message: message.into(),
        }
    }

    /// Create a network error
    pub fn network(message: impl Into<String>) -> Self {
        Self::Network {
            message: message.into(),
        }
    }

    /// Create an internal error
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
            Self::CommandFailed { .. } => 500,
        }
    }
}

/// Pool information with comprehensive metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolInfo {
    /// Name of the ZFS pool
    pub name: String,
    /// Current health status of the pool
    pub health: PoolHealth,
    /// Current operational state of the pool
    pub state: PoolState,
    /// Storage capacity information
    pub capacity: PoolCapacity,
    /// List of devices in the pool
    pub devices: Vec<String>,
    /// ZFS properties set on the pool
    pub properties: HashMap<String, String>,
    /// Timestamp when the pool was created
    pub created_at: SystemTime,
    /// Timestamp of the last scrub operation
    pub last_scrub: Option<SystemTime>,
    /// Current status of scrub operations
    pub scrub_status: ScrubStatus,
    /// List of any errors encountered
    pub errors: Vec<String>,
}

/// Pool health status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PoolHealth {
    /// Pool is healthy and fully operational
    Online,
    /// Pool is operational but with reduced redundancy
    Degraded,
    /// Pool has critical errors and may be unavailable
    Faulted,
    /// Pool is intentionally offline
    Offline,
    /// Pool is unavailable due to system issues
    Unavailable,
    /// Pool has been removed from the system
    Removed,
    /// Pool health status cannot be determined
    Unknown,
}

/// Pool state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PoolState {
    /// Pool is active and available for use
    Active,
    /// Pool has been exported and is not available
    Exported,
    /// Pool has been destroyed
    Destroyed,
    /// Pool device is used as a spare
    Spare,
    /// Pool device is used as L2 cache
    L2Cache,
    /// Pool is temporarily unavailable
    Unavailable,
    /// Pool state cannot be determined
    Unknown,
}

/// Pool capacity information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolCapacity {
    /// Total capacity of the pool in bytes
    pub total_bytes: u64,
    /// Used space in the pool in bytes
    pub used_bytes: u64,
    /// Available space in the pool in bytes
    pub available_bytes: u64,
    /// Pool utilization as a percentage (0.0-100.0)
    pub utilization_percent: f64,
}

/// Pool scrub status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ScrubStatus {
    /// No scrub has been performed
    None,
    /// Scrub is currently in progress
    InProgress {
        /// Percentage of scrub completion
        percent_complete: f64,
    },
    /// Scrub has completed successfully
    Completed {
        /// Number of errors found during scrub
        errors_found: u32,
    },
    /// Scrub was cancelled before completion
    Cancelled,
    /// Scrub failed to complete
    Failed {
        /// Reason for scrub failure
        reason: String,
    },
}

/// Dataset information with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetInfo {
    /// Name of the dataset
    pub name: String,
    /// Type of the dataset (filesystem, volume, etc.)
    pub dataset_type: DatasetType,
    /// Used space in the dataset in bytes
    pub used_space: u64,
    /// Available space in the dataset in bytes
    pub available_space: u64,
    /// Mount point for the dataset (if applicable)
    pub mount_point: Option<String>,
    /// ZFS properties set on the dataset
    pub properties: HashMap<String, String>,
    /// Timestamp when the dataset was created
    pub created_at: SystemTime,
    /// Parent dataset name (if any)
    pub parent: Option<String>,
    /// Child dataset names
    pub children: Vec<String>,
}

/// Dataset type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DatasetType {
    /// ZFS filesystem dataset
    Filesystem,
    /// ZFS block device volume
    Volume,
    /// ZFS snapshot dataset
    Snapshot,
    /// ZFS bookmark reference
    Bookmark,
}

/// Snapshot information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotInfo {
    /// Name of the snapshot
    pub name: String,
    /// Dataset that was snapshotted
    pub dataset: String,
    /// Timestamp when the snapshot was created
    pub created_at: SystemTime,
    /// Size of the snapshot in bytes
    pub size_bytes: u64,
    /// ZFS properties set on the snapshot
    pub properties: HashMap<String, String>,
    /// Optional description of the snapshot
    pub description: Option<String>,
}

/// Configuration for creating ZFS pools
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolConfig {
    /// Name for the new pool
    pub name: String,
    /// List of devices to include in the pool
    pub devices: Vec<String>,
    /// Optional RAID level specification
    pub raid_level: Option<String>,
    /// ZFS properties to set on the pool
    pub properties: HashMap<String, String>,
}

/// Configuration for creating ZFS datasets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetConfig {
    /// Name of the dataset to create
    pub name: String,
    /// Optional parent dataset
    pub parent: Option<String>,
    /// Type of dataset (filesystem, volume, etc.)
    pub dataset_type: DatasetType,
    /// ZFS properties to set on the dataset
    pub properties: HashMap<String, String>,
}

/// Configuration for creating ZFS snapshots
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotConfig {
    /// Name of the snapshot
    pub name: String,
    /// Dataset to snapshot
    pub dataset: String,
    /// Optional description of the snapshot
    pub description: Option<String>,
    /// ZFS properties to set on the snapshot
    pub properties: HashMap<String, String>,
}

/// Service health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    /// Name of the service being monitored
    pub service_name: String,
    /// Current service status
    pub status: ServiceStatus,
    /// Timestamp of the last health check
    pub last_check: SystemTime,
    /// Whether ZFS kernel module is available
    pub zfs_available: bool,
    /// Whether all ZFS pools are healthy
    pub pools_healthy: bool,
    /// Whether all datasets are healthy
    pub datasets_healthy: bool,
    /// Overall system health status
    pub system_healthy: bool,
    /// Detailed health check results
    pub checks: Vec<HealthCheck>,
    /// Optional service performance metrics
    pub metrics: Option<ServiceMetrics>,
}

/// Service status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ServiceStatus {
    /// Service is operating normally
    Healthy,
    /// Service is functional but with reduced performance
    Degraded,
    /// Service is not functioning properly
    Unhealthy,
    /// Service status cannot be determined
    Unknown,
}

/// Individual health check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    /// Name of the health check
    pub name: String,
    /// Result status of the health check
    pub status: ServiceStatus,
    /// Descriptive message about the check result
    pub message: String,
    /// Time taken to perform the health check
    pub duration: Duration,
}

/// Service metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMetrics {
    /// Name of the service these metrics belong to
    pub service_name: String,
    /// Timestamp when metrics were collected
    pub timestamp: SystemTime,
    /// Service uptime duration
    pub uptime: Duration,
    /// Total number of requests processed
    pub requests_total: u64,
    /// Number of successful requests
    pub requests_successful: u64,
    /// Number of failed requests
    pub requests_failed: u64,
    /// Average response time for requests
    pub average_response_time: Duration,
    /// Error rate as a percentage (0.0-1.0)
    pub error_rate: f64,
    /// Current circuit breaker state
    pub circuit_breaker_state: String,
    /// Number of active connections
    pub active_connections: u32,
    /// Additional custom metrics
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

// ==================== SECTION ====================

impl From<std::io::Error> for UniversalZfsError {
    fn from(error: std::io::Error) -> Self {
        UniversalZfsError::Backend {
            backend: "system".to_string(),
            message: format!("IO error: {}", error),
        }
    }
}

impl From<tokio::time::error::Elapsed> for UniversalZfsError {
    fn from(_error: tokio::time::error::Elapsed) -> Self {
        UniversalZfsError::Timeout {
            operation: "command execution".to_string(),
            duration: Duration::from_secs(30), // Default timeout
        }
    }
}
