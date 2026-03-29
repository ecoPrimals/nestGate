// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **RETRY STRATEGY**
//!
//! Comprehensive retry mechanisms with various backoff strategies.

use crate::error::NestGateError;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, warn};

#[inline]
fn duration_millis_clamped(d: Duration) -> u64 {
    u64::try_from(d.as_millis().min(u128::from(u64::MAX))).unwrap_or(u64::MAX)
}

#[inline]
fn f64_to_delay_ms(x: f64) -> u64 {
    if !x.is_finite() || x <= 0.0 {
        return 0;
    }
    if x >= u64::MAX as f64 {
        return u64::MAX;
    }
    #[expect(
        clippy::cast_possible_truncation,
        reason = "delay milliseconds clamped to u64 range"
    )]
    #[expect(
        clippy::cast_sign_loss,
        reason = "non-finite and non-positive delays mapped to 0 above"
    )]
    let v: u64 = x as u64;
    v
}

/// Retry configuration
#[derive(Debug, Clone)]
/// Configuration for Retry
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_attempts: u32,
    /// Initial delay between retries
    pub initial_delay: Duration,
    /// Maximum delay between retries
    pub max_delay: Duration,
    /// Backoff multiplier
    pub backoff_multiplier: f64,
    /// Whether to add jitter to delays
    pub jitter: bool,
}

impl Default for RetryConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 2.0,
            jitter: true,
        }
    }
}

/// Retry strategy trait
pub trait RetryStrategy: Send + Sync {
    /// Calculate delay for the given attempt number (0-based)
    fn delay(&self, attempt: u32) -> Duration;

    /// Check if should retry for the given attempt and error
    fn should_retry(&self, attempt: u32, error: &NestGateError) -> bool;
}

/// Exponential backoff retry strategy
#[derive(Debug, Clone)]
/// Exponentialbackoff
pub struct ExponentialBackoff {
    config: RetryConfig,
}

impl ExponentialBackoff {
    /// Create a new exponential backoff strategy
    #[must_use]
    pub const fn new(config: RetryConfig) -> Self {
        Self { config }
    }

    /// Create with default configuration
    #[must_use]
    pub fn default_config() -> Self {
        Self::new(RetryConfig::default())
    }
}

impl RetryStrategy for ExponentialBackoff {
    /// Delay
    fn delay(&self, attempt: u32) -> Duration {
        let base_delay_ms = self.config.initial_delay.as_secs_f64() * 1000.0;
        let multiplier = self.config.backoff_multiplier.powi(attempt as i32);
        let delay_ms = f64_to_delay_ms(base_delay_ms * multiplier);

        let max_ms = duration_millis_clamped(self.config.max_delay);
        let delay = Duration::from_millis(delay_ms.min(max_ms));

        if self.config.jitter {
            // Add ±25% jitter
            let delay_ms_f = delay.as_secs_f64() * 1000.0;
            let jitter_range = delay_ms_f * 0.25;
            let jitter = (fastrand::f64() - 0.5) * 2.0 * jitter_range;
            let jittered_delay = f64_to_delay_ms((delay_ms_f + jitter).max(0.0));
            Duration::from_millis(jittered_delay.min(max_ms))
        } else {
            delay
        }
    }

    /// Should Retry
    fn should_retry(&self, attempt: u32, error: &NestGateError) -> bool {
        if attempt + 1 >= self.config.max_attempts {
            return false;
        }

        // Determine if error is retryable
        match error {
            NestGateError::Network(_) | NestGateError::Timeout(_) | NestGateError::Internal(_) => {
                true
            }
            NestGateError::Validation(_)
            | NestGateError::Security(_)
            | NestGateError::Api(_)
            | _ => false, // Don't retry validation/security/API; conservative for other errors
        }
    }
}

/// Linear backoff retry strategy
#[derive(Debug, Clone)]
/// Linearbackoff
pub struct LinearBackoff {
    config: RetryConfig,
}

impl LinearBackoff {
    /// Create a new linear backoff strategy
    #[must_use]
    pub const fn new(config: RetryConfig) -> Self {
        Self { config }
    }
}

impl RetryStrategy for LinearBackoff {
    /// Delay
    fn delay(&self, attempt: u32) -> Duration {
        let base_ms = duration_millis_clamped(self.config.initial_delay);
        let delay_ms = base_ms.saturating_mul(u64::from(attempt + 1));
        let max_ms = duration_millis_clamped(self.config.max_delay);
        let delay = Duration::from_millis(delay_ms.min(max_ms));

        if self.config.jitter {
            let delay_ms_f = delay.as_secs_f64() * 1000.0;
            let jitter_range = delay_ms_f * 0.1;
            let jitter = (fastrand::f64() - 0.5) * 2.0 * jitter_range;
            let jittered_delay = f64_to_delay_ms((delay_ms_f + jitter).max(0.0));
            Duration::from_millis(jittered_delay.min(max_ms))
        } else {
            delay
        }
    }

    /// Should Retry
    fn should_retry(&self, attempt: u32, error: &NestGateError) -> bool {
        if attempt + 1 >= self.config.max_attempts {
            return false;
        }

        // Same retry logic as exponential backoff
        matches!(
            error,
            NestGateError::Network(_) | NestGateError::Timeout(_) | NestGateError::Internal(_)
        )
    }
}

/// Retry executor
pub struct RetryExecutor<S: RetryStrategy> {
    strategy: S,
    operation_name: String,
}

impl<S: RetryStrategy> RetryExecutor<S> {
    /// Create a new retry executor
    pub const fn new(strategy: S, operation_name: String) -> Self {
        Self {
            strategy,
            operation_name,
        }
    }

    /// Execute an operation with retry logic
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn execute<F, Fut, T>(&self, operation: F) -> Result<T, NestGateError>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, NestGateError>>,
    {
        let mut attempt = 0;
        let mut _last_error = None;

        loop {
            debug!(
                "Executing {} (attempt {})",
                self.operation_name,
                attempt + 1
            );

            match operation().await {
                Ok(result) => {
                    if attempt > 0 {
                        debug!(
                            "Operation {} succeeded after {} attempts",
                            self.operation_name,
                            attempt + 1
                        );
                    }
                    return Ok(result);
                }
                Err(error) => {
                    if !self.strategy.should_retry(attempt, &error) {
                        warn!(
                            "Operation {} failed after {} attempts: {}",
                            self.operation_name,
                            attempt + 1,
                            error
                        );
                        return Err(error);
                    }

                    let delay = self.strategy.delay(attempt);
                    debug!(
                        "Operation {} failed (attempt {}), retrying in {:?}: {}",
                        self.operation_name,
                        attempt + 1,
                        delay,
                        error
                    );

                    _last_error = Some(error);
                    sleep(delay).await;
                    attempt += 1;
                }
            }
        }
    }
}

/// Convenience function for exponential backoff retry
pub async fn retry_with_exponential_backoff<F, Fut, T>(
    operation: F,
    config: RetryConfig,
    operation_name: &str,
) -> Result<T, NestGateError>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T, NestGateError>>,
{
    let strategy = ExponentialBackoff::new(config);
    let executor = RetryExecutor::new(strategy, operation_name.to_string());
    executor.execute(operation).await
}

/// Convenience function for linear backoff retry
pub async fn retry_with_linear_backoff<F, Fut, T>(
    operation: F,
    config: RetryConfig,
    operation_name: &str,
) -> Result<T, NestGateError>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T, NestGateError>>,
{
    let strategy = LinearBackoff::new(config);
    let executor = RetryExecutor::new(strategy, operation_name.to_string());
    executor.execute(operation).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU32, Ordering};

    #[tokio::test]
    async fn test_exponential_backoff_success() {
        let config = RetryConfig {
            max_attempts: 3,
            initial_delay: Duration::from_millis(10),
            max_delay: Duration::from_secs(1),
            backoff_multiplier: 2.0,
            jitter: false,
        };

        let attempt_count = Arc::new(AtomicU32::new(0));
        let attempt_count_clone = attempt_count.clone();

        let result = retry_with_exponential_backoff(
            || {
                let count = attempt_count_clone.fetch_add(1, Ordering::SeqCst);
                async move {
                    if count < 2 {
                        Err(NestGateError::Network(Box::new(
                            crate::error::variants::core_errors::NetworkErrorDetails {
                                message: "Network error".to_string(),
                                operation: None,
                                endpoint: None,
                                network_data: None,
                                context: None,
                            },
                        )))
                    } else {
                        Ok("success")
                    }
                }
            },
            config,
            "test_operation",
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(result.expect("Operation failed"), "success");
        assert_eq!(attempt_count.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_exponential_backoff_failure() {
        let config = RetryConfig {
            max_attempts: 2,
            initial_delay: Duration::from_millis(10),
            max_delay: Duration::from_secs(1),
            backoff_multiplier: 2.0,
            jitter: false,
        };

        let attempt_count = Arc::new(AtomicU32::new(0));
        let attempt_count_clone = attempt_count.clone();

        let result: Result<(), _> = retry_with_exponential_backoff(
            || {
                attempt_count_clone.fetch_add(1, Ordering::SeqCst);
                async move {
                    Err(NestGateError::Network(Box::new(
                        crate::error::variants::core_errors::NetworkErrorDetails {
                            message: "Network error".to_string(),
                            operation: None,
                            endpoint: None,
                            network_data: None,
                            context: None,
                        },
                    )))
                }
            },
            config,
            "test_operation",
        )
        .await;

        assert!(result.is_err());
        assert_eq!(attempt_count.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn test_non_retryable_error() {
        let config = RetryConfig::default();

        let attempt_count = Arc::new(AtomicU32::new(0));
        let attempt_count_clone = attempt_count.clone();

        let result: Result<(), _> = retry_with_exponential_backoff(
            || {
                attempt_count_clone.fetch_add(1, Ordering::SeqCst);
                async move {
                    Err(NestGateError::Validation(Box::new(
                        crate::error::variants::core_errors::ValidationErrorDetails {
                            message: "Invalid input".to_string(),
                            field: Some("test_field".to_string()),
                            expected: None,
                            actual: None,
                            context: None,
                        },
                    )))
                }
            },
            config,
            "test_operation",
        )
        .await;

        assert!(result.is_err());
        assert_eq!(attempt_count.load(Ordering::SeqCst), 1); // Should not retry
    }

    #[test]
    fn test_exponential_backoff_delay() {
        let config = RetryConfig {
            max_attempts: 5,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            backoff_multiplier: 2.0,
            jitter: false,
        };

        let strategy = ExponentialBackoff::new(config);

        assert_eq!(strategy.delay(0), Duration::from_millis(100));
        assert_eq!(strategy.delay(1), Duration::from_millis(200));
        assert_eq!(strategy.delay(2), Duration::from_millis(400));
        assert_eq!(strategy.delay(3), Duration::from_millis(800));
    }

    #[test]
    fn test_linear_backoff_delay() {
        let config = RetryConfig {
            max_attempts: 5,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            backoff_multiplier: 1.0, // Not used in linear backoff
            jitter: false,
        };

        let strategy = LinearBackoff::new(config);

        assert_eq!(strategy.delay(0), Duration::from_millis(100));
        assert_eq!(strategy.delay(1), Duration::from_millis(200));
        assert_eq!(strategy.delay(2), Duration::from_millis(300));
        assert_eq!(strategy.delay(3), Duration::from_millis(400));
    }
}
