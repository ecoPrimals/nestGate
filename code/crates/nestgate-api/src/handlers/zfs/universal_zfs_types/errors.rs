// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Universal ZFS Error Types
//!
//! Comprehensive error handling for ZFS operations across all backend implementations.

use nestgate_core::error::NestGateError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::result::Result;
use std::time::Duration;
use std::time::SystemTime;
use thiserror::Error;

/// **CANONICAL**: Universal ZFS Result type using Result
/// This follows the canonical Result<T,E> pattern with domain-specific error type
pub type UniversalZfsResult<T> = Result<T, UniversalZfsError>;

/// Error data structure for universal ZFS operations
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Universalzfserrordata
pub struct UniversalZfsErrorData {
    /// Human-readable error message describing what went wrong
    pub message: String,
    /// The specific operation that failed (e.g., "`create_pool`", "snapshot")
    pub b_operation: Option<String>,
    /// The backend service that encountered the error (e.g., "local", "remote")
    pub backend: Option<String>,
    /// Path to the resource involved in the error (e.g., filesystem path, dataset name)
    pub path: Option<String>,
    /// Duration after which the operation timed out, if applicable
    pub timeout_duration: Option<Duration>,
    /// Whether the error was caused by an open circuit breaker
    pub circuit_breaker_open: bool,
    /// Rate limiting information if the error was due to rate limiting
    pub rate_limit_info: Option<RateLimitInfo>,
}

/// Rate limiting information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Ratelimitinfo
pub struct RateLimitInfo {
    /// Maximum number of requests allowed within the time window
    pub limit: u32,
    /// Time window for the rate limit (e.g., `Duration::from_secs(60)` for per-minute limit)
    pub window: Duration,
    /// Current number of requests made within the current window
    pub current_usage: u32,
    /// System time when the rate limit counter will reset
    pub reset_time: SystemTime,
}

/// Comprehensive error types for universal ZFS operations
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
/// Errors that can occur during `UniversalZfs` operations
pub enum UniversalZfsError {
    #[error("Service unavailable: {message}")]
    /// Service is temporarily unavailable
    ServiceUnavailable {
        /// Error message describing the unavailability
        message: String,
    },
    #[error("Operation timeout after {duration:?}: {b_operation}")]
    /// Operation timed out
    Timeout {
        /// Operation that timed out
        b_operation: String,
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
    /// Backend service error
    Backend {
        /// Backend service name
        backend: String,
        /// Error message
        message: String,
    },
    #[error("Invalid input: {message}")]
    /// Invalid input provided
    InvalidInput {
        /// Error message
        message: String,
    },
    #[error("Resource not found: {path}")]
    /// Resource not found
    NotFound {
        /// Path to the resource
        path: String,
    },
    #[error("Circuit breaker open for {backend}")]
    /// Circuit breaker is open
    CircuitBreakerOpen {
        /// Backend service name
        backend: String,
    },
    #[error("Rate limit exceeded: {message}")]
    /// Rate limit exceeded
    RateLimitExceeded {
        /// Rate limit error message
        message: String,
        /// Rate limit information
        rate_limit_info: Option<RateLimitInfo>,
    },
    #[error("Internal error: {message}")]
    /// Internal system error
    Internal {
        /// Error message
        message: String,
    },
    #[error("Pool operation failed: {message}")]
    /// Pool-specific operation failed
    PoolOperationFailed {
        /// Error message
        message: String,
    },
    #[error("Dataset operation failed: {message}")]
    /// Dataset-specific operation failed
    DatasetOperationFailed {
        /// Error message
        message: String,
    },
    #[error("Snapshot operation failed: {message}")]
    /// Snapshot-specific operation failed
    SnapshotOperationFailed {
        /// Error message
        message: String,
    },
}

impl UniversalZfsError {
    /// Create a `ServiceUnavailable` error
    pub fn service_unavailable(message: impl Into<String>) -> Self {
        Self::ServiceUnavailable {
            message: message.into(),
        }
    }

    /// Create a Timeout error
    pub fn timeout(b_operation: impl Into<String>, duration: Duration) -> Self {
        Self::Timeout {
            b_operation: b_operation.into(),
            duration,
        }
    }

    /// Create a Configuration error
    pub fn configuration(message: impl Into<String>) -> Self {
        Self::Configuration {
            message: message.into(),
        }
    }

    /// Create a Backend error
    pub fn backend(backend: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Backend {
            backend: backend.into(),
            message: message.into(),
        }
    }

    /// Create an `InvalidInput` error
    pub fn invalid_input(message: impl Into<String>) -> Self {
        Self::InvalidInput {
            message: message.into(),
        }
    }

    /// Create a `NotFound` error
    pub fn not_found(path: impl Into<String>) -> Self {
        Self::NotFound { path: path.into() }
    }

    /// Create a `CircuitBreakerOpen` error
    pub fn circuit_breaker_open(backend: impl Into<String>) -> Self {
        Self::CircuitBreakerOpen {
            backend: backend.into(),
        }
    }

    /// Create a `RateLimitExceeded` error
    pub fn rate_limit_exceeded(
        message: impl Into<String>,
        rate_limit_info: Option<RateLimitInfo>,
    ) -> Self {
        Self::RateLimitExceeded {
            message: message.into(),
            rate_limit_info,
        }
    }

    /// Create an Internal error
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal {
            message: message.into(),
        }
    }

    /// Convert error to `UniversalZfsErrorData`
    #[must_use]
    pub fn to_error_data(&self) -> UniversalZfsErrorData {
        #[expect(
            clippy::missing_const_for_fn,
            reason = "String parameters are not valid in const fn"
        )]
        fn base_message(message: String) -> UniversalZfsErrorData {
            UniversalZfsErrorData {
                message,
                b_operation: None,
                backend: None,
                path: None,
                timeout_duration: None,
                circuit_breaker_open: false,
                rate_limit_info: None,
            }
        }

        fn with_operation(message: String, op: &'static str) -> UniversalZfsErrorData {
            UniversalZfsErrorData {
                message,
                b_operation: Some(op.to_string()),
                backend: None,
                path: None,
                timeout_duration: None,
                circuit_breaker_open: false,
                rate_limit_info: None,
            }
        }

        match self {
            Self::ServiceUnavailable { message }
            | Self::Configuration { message }
            | Self::InvalidInput { message }
            | Self::Internal { message } => base_message(message.clone()),
            Self::Timeout {
                b_operation,
                duration,
            } => UniversalZfsErrorData {
                message: format!("Operation timed out: {b_operation}"),
                b_operation: Some(b_operation.clone()),
                backend: None,
                path: None,
                timeout_duration: Some(*duration),
                circuit_breaker_open: false,
                rate_limit_info: None,
            },
            Self::Backend { backend, message } => UniversalZfsErrorData {
                message: message.clone(),
                b_operation: None,
                backend: Some(backend.clone()),
                path: None,
                timeout_duration: None,
                circuit_breaker_open: false,
                rate_limit_info: None,
            },
            Self::NotFound { path } => UniversalZfsErrorData {
                message: format!("Resource not found: {path}"),
                b_operation: None,
                backend: None,
                path: Some(path.clone()),
                timeout_duration: None,
                circuit_breaker_open: false,
                rate_limit_info: None,
            },
            Self::CircuitBreakerOpen { backend } => UniversalZfsErrorData {
                message: format!("Circuit breaker open for {backend}"),
                b_operation: None,
                backend: Some(backend.clone()),
                path: None,
                timeout_duration: None,
                circuit_breaker_open: true,
                rate_limit_info: None,
            },
            Self::RateLimitExceeded {
                message,
                rate_limit_info,
            } => UniversalZfsErrorData {
                message: message.clone(),
                b_operation: None,
                backend: None,
                path: None,
                timeout_duration: None,
                circuit_breaker_open: false,
                rate_limit_info: rate_limit_info.clone(),
            },
            Self::PoolOperationFailed { message } => {
                with_operation(message.clone(), "pool_operation")
            }
            Self::DatasetOperationFailed { message } => {
                with_operation(message.clone(), "dataset_operation")
            }
            Self::SnapshotOperationFailed { message } => {
                with_operation(message.clone(), "snapshot_operation")
            }
        }
    }
}

// ==================== ERROR CONVERSIONS ====================

impl From<std::io::Error> for UniversalZfsError {
    /// From
    fn from(_error: std::io::Error) -> Self {
        Self::Backend {
            backend: "system".to_string(),
            message: "IO error: self.base_url".to_string(),
        }
    }
}

impl From<tokio::time::error::Elapsed> for UniversalZfsError {
    /// From
    fn from(_error: tokio::time::error::Elapsed) -> Self {
        Self::Timeout {
            b_operation: "operation".to_string(),
            duration: Duration::from_secs(30), // Default timeout
        }
    }
}

impl From<UniversalZfsError> for NestGateError {
    /// From
    fn from(error: UniversalZfsError) -> Self {
        let error_data = error.to_error_data();
        let _details = HashMap::from([
            ("message".to_string(), error_data.message.clone()),
            (
                "backend".to_string(),
                error_data.backend.unwrap_or_else(|| "unknown".to_string()),
            ),
        ]);

        match error {
            UniversalZfsError::ServiceUnavailable { .. }
            | UniversalZfsError::CircuitBreakerOpen { .. }
            | UniversalZfsError::RateLimitExceeded { .. } => {
                Self::service_unavailable(error_data.message)
            }
            UniversalZfsError::Timeout {
                b_operation,
                duration,
            } => Self::timeout_error(b_operation, duration),
            UniversalZfsError::Configuration { .. } => {
                Self::configuration_error("zfs", error_data.message)
            }
            UniversalZfsError::Backend { backend, .. } => {
                Self::external_service_unavailable(backend, error_data.message)
            }
            UniversalZfsError::InvalidInput { .. } => Self::validation_error(error_data.message),
            UniversalZfsError::NotFound { .. } => Self::not_found(error_data.message),
            UniversalZfsError::Internal { .. } => Self::internal(error_data.message),
            UniversalZfsError::PoolOperationFailed { .. }
            | UniversalZfsError::DatasetOperationFailed { .. }
            | UniversalZfsError::SnapshotOperationFailed { .. } => {
                Self::storage_error(error_data.message)
            }
        }
    }
}

#[cfg(test)]
mod universal_zfs_error_tests {
    use super::*;
    use nestgate_core::error::NestGateError;
    use std::time::Duration;

    #[test]
    fn constructors_and_to_error_data_cover_variants() {
        let reset = SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000);
        let rli = RateLimitInfo {
            limit: 10,
            window: Duration::from_secs(60),
            current_usage: 3,
            reset_time: reset,
        };

        let cases: Vec<UniversalZfsError> = vec![
            UniversalZfsError::service_unavailable("svc"),
            UniversalZfsError::timeout("op", Duration::from_millis(12)),
            UniversalZfsError::configuration("cfg"),
            UniversalZfsError::backend("b", "m"),
            UniversalZfsError::invalid_input("bad"),
            UniversalZfsError::not_found("/p"),
            UniversalZfsError::circuit_breaker_open("cb"),
            UniversalZfsError::rate_limit_exceeded("rl", Some(rli.clone())),
            UniversalZfsError::internal("in"),
            UniversalZfsError::PoolOperationFailed {
                message: "p".to_string(),
            },
            UniversalZfsError::DatasetOperationFailed {
                message: "d".to_string(),
            },
            UniversalZfsError::SnapshotOperationFailed {
                message: "s".to_string(),
            },
        ];

        for err in cases {
            let data = err.to_error_data();
            assert!(!data.message.is_empty());
            let _: UniversalZfsErrorData = data;
        }

        let rl_only_msg = UniversalZfsError::rate_limit_exceeded("x", None);
        assert!(rl_only_msg.to_error_data().rate_limit_info.is_none());
    }

    #[test]
    fn from_io_and_tokio_elapsed() {
        let io_err = std::io::Error::other("disk");
        let u1: UniversalZfsError = io_err.into();
        assert!(matches!(u1, UniversalZfsError::Backend { .. }));

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .build()
            .expect("runtime");
        rt.block_on(async {
            let res =
                tokio::time::timeout(Duration::from_nanos(1), std::future::pending::<()>()).await;
            let elapsed = res.expect_err("timeout");
            let u2: UniversalZfsError = elapsed.into();
            assert!(matches!(u2, UniversalZfsError::Timeout { .. }));
        });
    }

    #[test]
    fn into_nest_gate_error_maps_all_variants() {
        let reset = SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000);
        let rli = RateLimitInfo {
            limit: 1,
            window: Duration::from_secs(1),
            current_usage: 0,
            reset_time: reset,
        };

        let _: NestGateError = UniversalZfsError::service_unavailable("x").into();
        let _: NestGateError = UniversalZfsError::circuit_breaker_open("b").into();
        let _: NestGateError = UniversalZfsError::rate_limit_exceeded("m", Some(rli)).into();
        let _: NestGateError = UniversalZfsError::timeout("op", Duration::from_secs(1)).into();
        let _: NestGateError = UniversalZfsError::configuration("c").into();
        let _: NestGateError = UniversalZfsError::backend("be", "msg").into();
        let _: NestGateError = UniversalZfsError::invalid_input("i").into();
        let _: NestGateError = UniversalZfsError::not_found("/n").into();
        let _: NestGateError = UniversalZfsError::internal("i").into();
        let _: NestGateError = UniversalZfsError::PoolOperationFailed {
            message: "p".to_string(),
        }
        .into();
        let _: NestGateError = UniversalZfsError::DatasetOperationFailed {
            message: "d".to_string(),
        }
        .into();
        let _: NestGateError = UniversalZfsError::SnapshotOperationFailed {
            message: "s".to_string(),
        }
        .into();
    }
}
