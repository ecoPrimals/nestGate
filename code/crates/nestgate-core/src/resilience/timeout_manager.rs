use crate::error::NestGateError;
//
// Provides timeout functionality for operations to prevent hanging requests
// and ensure system responsiveness.

use crate::{Result};
use std::time::Duration;

/// Execute operation with timeout
where
    F: Future<Output = Result<T>>,
{
    match tokio::time::timeout(timeout, operation).await {
        Ok(result) => result,
        Err(_) => Err(NestGateError::simple("Operation timed out")),
    }
}
/// Timeout configuration
#[derive(Debug, Clone)]
/// Configuration for Timeout
pub struct TimeoutConfig {
    /// Default Timeout
    pub default_timeout: Duration,
    /// Connection Timeout
    pub connection_timeout: Duration,
    /// Request Timeout
    pub request_timeout: Duration,
    /// Long Operation Timeout
    pub long_operation_timeout: Duration,
}
impl Default for TimeoutConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            default_timeout: Duration::from_secs(30),
            connection_timeout: Duration::from_secs(10),
            request_timeout: Duration::from_secs(60),
            long_operation_timeout: Duration::from_secs(300),
        }
    }
}

/// Timeout manager for different operation types
pub struct TimeoutManager {
    config: TimeoutConfig,
}
impl TimeoutManager {
    /// Creates a new instance
    pub fn new(config: TimeoutConfig) -> Self {
        Self { config }
    }

    /// Execute with default timeout
    where
        F: Future<Output = Result<T>>,
    {
        execute_with_timeout(operation, self.config.default_timeout).await
    }

    /// Execute with connection timeout
    where
        F: Future<Output = Result<T>>,
    {
        execute_with_timeout(operation, self.config.connection_timeout).await
    }

    /// Execute with request timeout
    where
        F: Future<Output = Result<T>>,
    {
        execute_with_timeout(operation, self.config.request_timeout).await
    }

    /// Execute with long operation timeout
    where
        F: Future<Output = Result<T>>,
    {
        execute_with_timeout(operation, self.config.long_operation_timeout).await
    }
}
