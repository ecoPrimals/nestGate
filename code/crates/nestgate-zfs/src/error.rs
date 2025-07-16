//! Error types for NestGate ZFS
//!
//! Comprehensive error handling for all ZFS operations

use std::fmt;
use thiserror::Error;

/// Main ZFS error type
#[derive(Error, Debug)]
pub enum ZfsError {
    /// Pool-related errors
    #[error("Pool error: {0}")]
    PoolError(#[from] PoolError),

    /// Dataset-related errors
    #[error("Dataset error: {0}")]
    DatasetError(#[from] DatasetError),

    /// Snapshot-related errors
    #[error("Snapshot error: {0}")]
    SnapshotError(#[from] SnapshotError),

    /// Migration-related errors
    #[error("Migration error: {0}")]
    MigrationError(#[from] MigrationError),

    /// AI-related errors
    #[error("AI error: {0}")]
    AiError(#[from] AiError),

    /// Performance-related errors
    #[error("Performance error: {0}")]
    PerformanceError(#[from] PerformanceError),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// System-level errors
    #[error("System unavailable: {0}")]
    SystemUnavailable(String),

    /// Timeout errors
    #[error("Operation timed out: {0}")]
    Timeout(String),

    /// Resource exhaustion
    #[error("Resource exhausted: {0}")]
    ResourceExhausted(String),

    /// Network-related errors
    #[error("Network error: {0}")]
    Network(String),

    /// Permission errors
    #[error("Permission denied: {0}")]
    PermissionError(String),

    /// I/O errors
    #[error("I/O error: {0}")]
    IoError(std::io::Error),

    /// Serialization errors
    #[error("Serialization error: {0}")]
    SerializationError(serde_json::Error),

    /// Generic internal errors
    #[error("Internal error: {message}")]
    Internal { message: String },

    /// Storage-related errors
    #[error("Storage error: {message}")]
    Storage { message: String },

    /// Command execution errors
    #[error("Command failed: {command} - {error}")]
    CommandFailed { command: String, error: String },

    /// Feature not yet implemented
    #[error("Feature not implemented: {0}")]
    Unimplemented(String),

    /// Invalid schedule configuration
    #[error("Invalid schedule: {0}")]
    InvalidSchedule(String),

    /// Parse error
    #[error("Parse error: {0}")]
    ParseError(String),

    /// Dataset not found
    #[error("Dataset not found: {dataset}")]
    DatasetNotFound { dataset: String },

    /// Pool not found
    #[error("Pool not found: {pool}")]
    PoolNotFound { pool: String },

    /// Snapshot not found
    #[error("Snapshot not found: {snapshot}")]
    SnapshotNotFound { snapshot: String },

    /// Core error
    #[error("Core error: {0}")]
    CoreError(#[from] nestgate_core::NestGateError),

    /// Insufficient permissions
    #[error("Insufficient permissions: {operation}")]
    InsufficientPermissions { operation: String },

    /// Resource busy
    #[error("Resource busy: {resource}")]
    ResourceBusy { resource: String },

    /// Invalid argument
    #[error("Invalid argument: {argument} - {reason}")]
    InvalidArgument { argument: String, reason: String },

    /// Operation not supported
    #[error("Operation not supported: {operation}")]
    NotSupported { operation: String },

    /// Quota exceeded
    #[error("Quota exceeded: {quota_type}")]
    QuotaExceeded { quota_type: String },

    /// Dependency error
    #[error("Dependency error: {dependency} - {reason}")]
    DependencyError { dependency: String, reason: String },

    /// State error
    #[error("Invalid state: {current_state} - {expected_state}")]
    InvalidState {
        current_state: String,
        expected_state: String,
    },

    /// Validation error
    #[error("Validation failed: {field} - {reason}")]
    ValidationError { field: String, reason: String },

    /// Feature not available
    #[error("Feature not available: {feature}")]
    FeatureNotAvailable { feature: String },

    /// Network error
    #[error("Network error: {0}")]
    NetworkError(String),

    /// Authentication error
    #[error("Authentication error: {0}")]
    AuthError(String),

    /// Lock error
    #[error("Lock error: {resource}")]
    LockError { resource: String },
}

/// Pool-specific errors
#[derive(Error, Debug, Clone, PartialEq)]
pub enum PoolError {
    #[error("Pool not found: {pool_name}")]
    NotFound { pool_name: String },

    #[error("Pool already exists: {pool_name}")]
    AlreadyExists { pool_name: String },

    #[error("Pool is offline: {pool_name}")]
    Offline { pool_name: String },

    #[error("Pool is degraded: {pool_name}")]
    Degraded { pool_name: String },

    #[error("Insufficient space in pool: {pool_name}")]
    InsufficientSpace { pool_name: String },

    #[error("Pool creation failed for {pool_name}: {reason}")]
    CreationFailed { pool_name: String, reason: String },

    #[error("Pool destruction failed for {pool_name}: {reason}")]
    DestructionFailed { pool_name: String, reason: String },

    #[error("Pool discovery failed: {reason}")]
    DiscoveryFailed { reason: String },

    #[error("Pool health check failed: {pool_name} - {details}")]
    HealthCheckFailed { pool_name: String, details: String },

    #[error("Pool scrub failed for {pool_name}: {details}")]
    ScrubFailed { pool_name: String, details: String },
}

/// Dataset-specific errors
#[derive(Error, Debug, Clone, PartialEq)]
pub enum DatasetError {
    #[error("Dataset not found: {dataset_name}")]
    NotFound { dataset_name: String },

    #[error("Dataset already exists: {dataset_name}")]
    AlreadyExists { dataset_name: String },

    #[error("Dataset is busy: {dataset_name}")]
    Busy { dataset_name: String },

    #[error("Invalid dataset name: {dataset_name}")]
    InvalidName { dataset_name: String },

    #[error("Dataset creation failed: {reason}")]
    CreationFailed { reason: String },

    #[error("Dataset deletion failed: {reason}")]
    DeletionFailed { reason: String },

    #[error("Property operation failed: {reason}")]
    PropertyError { reason: String },
}

/// Snapshot-specific errors
#[derive(Error, Debug, Clone, PartialEq)]
pub enum SnapshotError {
    #[error("Snapshot not found: {snapshot_name}")]
    NotFound { snapshot_name: String },

    #[error("Snapshot already exists: {snapshot_name}")]
    AlreadyExists { snapshot_name: String },

    #[error("Snapshot creation failed for {dataset}@{snapshot}: {reason}")]
    CreationFailed {
        dataset: String,
        snapshot: String,
        reason: String,
    },

    #[error("Snapshot deletion failed for {dataset}@{snapshot}: {reason}")]
    DeletionFailed {
        dataset: String,
        snapshot: String,
        reason: String,
    },

    #[error("Snapshot rollback failed for {dataset}@{snapshot}: {reason}")]
    RollbackFailed {
        dataset: String,
        snapshot: String,
        reason: String,
    },

    #[error("Snapshot clone failed from {snapshot} to {clone_name}: {reason}")]
    CloneFailed {
        snapshot: String,
        clone_name: String,
        reason: String,
    },

    #[error("Snapshot send failed from {snapshot} to {destination}: {reason}")]
    SendFailed {
        snapshot: String,
        destination: String,
        reason: String,
    },

    #[error("Snapshot receive failed to {destination}: {reason}")]
    ReceiveFailed { destination: String, reason: String },

    #[error("Invalid parameters for {operation}: {reason}")]
    InvalidParameters { operation: String, reason: String },

    #[error("Policy execution failed: {reason}")]
    PolicyExecutionFailed { reason: String },

    #[error("Schedule parsing failed: {reason}")]
    ScheduleParsingFailed { reason: String },
}

/// Migration-specific errors
#[derive(Error, Debug, Clone, PartialEq)]
pub enum MigrationError {
    #[error("Migration job not found: {job_id}")]
    JobNotFound { job_id: String },

    #[error("Migration already in progress: {job_id}")]
    AlreadyInProgress { job_id: String },

    #[error("Migration failed: {reason}")]
    MigrationFailed { reason: String },

    #[error("Invalid migration request: {reason}")]
    InvalidRequest { reason: String },

    #[error("Migration quota exceeded")]
    QuotaExceeded,

    #[error("Source and destination tiers are the same")]
    SameTier,
}

/// AI integration errors
#[derive(Error, Debug, Clone, PartialEq)]
pub enum AiError {
    #[error("AI model not available: {model_name}")]
    ModelNotAvailable { model_name: String },

    #[error("AI prediction failed: {reason}")]
    PredictionFailed { reason: String },

    #[error("AI optimization failed: {reason}")]
    OptimizationFailed { reason: String },

    #[error("Insufficient data for AI analysis")]
    InsufficientData,

    #[error("AI confidence too low: {confidence}")]
    LowConfidence { confidence: f64 },

    #[error("AI service unavailable")]
    ServiceUnavailable,
}

/// Performance monitoring errors
#[derive(Error, Debug, Clone, PartialEq)]
pub enum PerformanceError {
    #[error("Metrics collection failed: {reason}")]
    CollectionFailed { reason: String },

    #[error("Alert condition invalid: {condition}")]
    InvalidAlertCondition { condition: String },

    #[error("Performance threshold exceeded: {metric}")]
    ThresholdExceeded { metric: String },

    #[error("Metrics storage full")]
    StorageFull,

    #[error("Analytics engine failed: {reason}")]
    AnalyticsFailed { reason: String },
}

/// Error context for better debugging
#[derive(Debug, Clone)]
pub struct ZfsErrorContext {
    pub operation: String,
    pub component: String,
    pub timestamp: std::time::SystemTime,
    pub additional_info: std::collections::HashMap<String, String>,
}

impl ZfsError {
    /// Check if an error is retryable
    pub fn is_retryable(&self) -> bool {
        match self {
            ZfsError::SystemUnavailable(_) => true,
            ZfsError::Timeout(_) => true,
            ZfsError::ResourceExhausted(_) => true,
            ZfsError::IoError(io_err) => matches!(
                io_err.kind(),
                std::io::ErrorKind::Interrupted
                    | std::io::ErrorKind::TimedOut
                    | std::io::ErrorKind::WouldBlock
            ),
            ZfsError::PoolError(PoolError::Offline { .. }) => true,
            ZfsError::DatasetError(DatasetError::Busy { .. }) => true,
            _ => false,
        }
    }

    /// Get error severity level
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            ZfsError::PoolError(PoolError::NotFound { .. }) => ErrorSeverity::Warning,
            ZfsError::PoolError(PoolError::Degraded { .. }) => ErrorSeverity::Critical,
            ZfsError::DatasetError(DatasetError::NotFound { .. }) => ErrorSeverity::Warning,
            ZfsError::SystemUnavailable(_) => ErrorSeverity::Critical,
            ZfsError::PermissionError(_) => ErrorSeverity::Error,
            ZfsError::ConfigError(_) => ErrorSeverity::Error,
            ZfsError::Timeout(_) => ErrorSeverity::Warning,
            ZfsError::ResourceExhausted(_) => ErrorSeverity::Critical,
            _ => ErrorSeverity::Error,
        }
    }

    /// Create error context
    pub fn create_context(operation: &str, component: &str) -> ZfsErrorContext {
        ZfsErrorContext {
            operation: operation.to_string(),
            component: component.to_string(),
            timestamp: std::time::SystemTime::now(),
            additional_info: std::collections::HashMap::new(),
        }
    }

    /// Convert to NestGate core error
    pub fn to_nestgate_error(&self) -> nestgate_core::NestGateError {
        match self {
            ZfsError::PoolError(PoolError::NotFound { pool_name }) => {
                nestgate_core::NestGateError::NotFound(format!("Pool not found: {}", pool_name))
            }
            ZfsError::DatasetError(DatasetError::NotFound { dataset_name }) => {
                nestgate_core::NestGateError::NotFound(format!(
                    "Dataset not found: {}",
                    dataset_name
                ))
            }
            ZfsError::PermissionError(msg) => {
                nestgate_core::NestGateError::Authorization(msg.clone())
            }
            ZfsError::ConfigError(msg) => nestgate_core::NestGateError::Configuration(msg.clone()),
            _ => nestgate_core::NestGateError::Internal(self.to_string()),
        }
    }
}

/// Convert ZfsError to NestGateError
impl From<ZfsError> for nestgate_core::NestGateError {
    fn from(error: ZfsError) -> Self {
        error.to_nestgate_error()
    }
}

/// Error severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorSeverity {
    Info = 0,
    Warning = 1,
    Error = 2,
    Critical = 3,
    Emergency = 4,
}

impl fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorSeverity::Info => write!(f, "INFO"),
            ErrorSeverity::Warning => write!(f, "WARNING"),
            ErrorSeverity::Error => write!(f, "ERROR"),
            ErrorSeverity::Critical => write!(f, "CRITICAL"),
            ErrorSeverity::Emergency => write!(f, "EMERGENCY"),
        }
    }
}

/// Result type alias for ZFS operations
pub type ZfsResult<T> = Result<T, ZfsError>;

/// Macro for creating ZFS errors with context
#[macro_export]
macro_rules! zfs_error {
    ($error_type:expr, $($arg:tt)*) => {
        ZfsError::from($error_type).with_context(format!($($arg)*))
    };
}

/// Trait for adding context to errors
pub trait ZfsErrorExt<T> {
    fn with_context(self, context: String) -> ZfsResult<T>;
    fn with_operation(self, operation: &str) -> ZfsResult<T>;
    fn with_component(self, component: &str) -> ZfsResult<T>;
}

impl<T> ZfsErrorExt<T> for ZfsResult<T> {
    fn with_context(self, context: String) -> ZfsResult<T> {
        self.map_err(|e| match e {
            ZfsError::Internal { message } => ZfsError::Internal {
                message: format!("{}: {}", context, message),
            },
            other => other,
        })
    }

    fn with_operation(self, operation: &str) -> ZfsResult<T> {
        self.with_context(format!("Operation: {}", operation))
    }

    fn with_component(self, component: &str) -> ZfsResult<T> {
        self.with_context(format!("Component: {}", component))
    }
}

impl From<std::io::Error> for ZfsError {
    fn from(err: std::io::Error) -> Self {
        ZfsError::IoError(err)
    }
}

impl From<serde_json::Error> for ZfsError {
    fn from(err: serde_json::Error) -> Self {
        ZfsError::SerializationError(err)
    }
}

// Add missing conversion for nestgate_core::NestGateError

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_retryability() {
        let retryable_errors = vec![
            ZfsError::SystemUnavailable("test".to_string()),
            ZfsError::Timeout("test".to_string()),
            ZfsError::ResourceExhausted("test".to_string()),
        ];

        for error in retryable_errors {
            assert!(
                error.is_retryable(),
                "Error should be retryable: {:?}",
                error
            );
        }

        let non_retryable_errors = vec![
            ZfsError::PoolError(PoolError::NotFound {
                pool_name: "test".to_string(),
            }),
            ZfsError::ConfigError("test".to_string()),
            ZfsError::PermissionError("test".to_string()),
        ];

        for error in non_retryable_errors {
            assert!(
                !error.is_retryable(),
                "Error should not be retryable: {:?}",
                error
            );
        }
    }

    #[test]
    fn test_error_severity() {
        let critical_error = ZfsError::SystemUnavailable("test".to_string());
        assert_eq!(critical_error.severity(), ErrorSeverity::Critical);

        let warning_error = ZfsError::PoolError(PoolError::NotFound {
            pool_name: "test".to_string(),
        });
        assert_eq!(warning_error.severity(), ErrorSeverity::Warning);
    }

    #[test]
    fn test_error_context() {
        let context = ZfsError::create_context("pool_creation", "pool_manager");
        assert_eq!(context.operation, "pool_creation");
        assert_eq!(context.component, "pool_manager");
    }
}
