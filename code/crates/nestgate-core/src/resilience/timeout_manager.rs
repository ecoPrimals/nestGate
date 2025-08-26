use crate::NestGateError;
//
// Provides timeout functionality for operations to prevent hanging requests
// and ensure system responsiveness.

use crate::{Result, NestGateError};
use std::time::Duration;

/// Execute operation with timeout
pub async fn execute_with_timeout<F, T>(operation: F, timeout: Duration) -> Result<T>
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
pub struct TimeoutConfig {
    pub default_timeout: Duration,
    pub connection_timeout: Duration,
    pub request_timeout: Duration,
    pub long_operation_timeout: Duration,
}

impl Default for TimeoutConfig {
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
    pub fn new(config: TimeoutConfig) -> Self {
        Self { config }
    }

    /// Execute with default timeout
    pub async fn execute_default<F, T>(&self, operation: F) -> Result<T>
    where
        F: Future<Output = Result<T>>,
    {
        execute_with_timeout(operation, self.config.default_timeout).await
    }

    /// Execute with connection timeout
    pub async fn execute_connection<F, T>(&self, operation: F) -> Result<T>
    where
        F: Future<Output = Result<T>>,
    {
        execute_with_timeout(operation, self.config.connection_timeout).await
    }

    /// Execute with request timeout
    pub async fn execute_request<F, T>(&self, operation: F) -> Result<T>
    where
        F: Future<Output = Result<T>>,
    {
        execute_with_timeout(operation, self.config.request_timeout).await
    }

    /// Execute with long operation timeout
    pub async fn execute_long_operation<F, T>(&self, operation: F) -> Result<T>
    where
        F: Future<Output = Result<T>>,
    {
        execute_with_timeout(operation, self.config.long_operation_timeout).await
    }
}
