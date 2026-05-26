// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// Provides retry logic for fail-safe operations.

//! Retry Executor module

use std::time::Duration;
// Removed unused tracing import

use crate::handlers::zfs::universal_zfs::config::RetryPolicy;
use crate::handlers::zfs::universal_zfs_types::UniversalZfsResult;
use tracing::debug;

/// Retry executor for operations
#[derive(Debug, Clone)]
/// Retryexecutor
pub struct RetryExecutor {
    config: RetryPolicy,
}
impl RetryExecutor {
    /// Create a new retry executor with the specified retry policy
    ///
    /// # Arguments
    /// * `config` - Retry policy configuration including max attempts and backoff settings
    ///
    /// # Returns
    /// * New retry executor instance
    #[must_use]
    pub const fn new(config: RetryPolicy) -> Self {
        Self { config }
    }

    /// Execute an operation with retry logic
    ///
    /// Attempts to execute the provided operation with exponential backoff
    /// retry logic according to the configured retry policy.
    ///
    /// # Arguments
    /// * `operation` - Async operation to execute with retry logic
    ///
    /// # Returns
    /// * Result of the operation after all retry attempts
    ///
    /// # Type Parameters
    /// * `Fut` - Future type that the operation returns
    /// * `F` - Function type that returns the future
    /// * `T` - Return type of the operation
    pub async fn execute<Fut, F, T>(&self, mut operation: F) -> UniversalZfsResult<T>
    where
        Fut: std::future::Future<Output = UniversalZfsResult<T>> + Send,
        F: FnMut() -> Fut,
    {
        let mut attempts = 0;
        let mut delay = self.config.initial_delay;

        loop {
            attempts += 1;

            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    if attempts >= self.config.max_attempts {
                        return Err(e);
                    }

                    debug!(
                        "Operation failed, retrying in {:?} (attempt {}/{})",
                        delay, attempts, self.config.max_attempts
                    );
                    tokio::time::sleep(delay).await;

                    delay = std::cmp::min(
                        Duration::from_millis(
                            (delay.as_millis() as f64 * self.config.backoff_multiplier) as u64,
                        ),
                        self.config.max_delay,
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handlers::zfs::universal_zfs_types::UniversalZfsError as Err;
    use std::sync::atomic::{AtomicU32, Ordering};

    fn fast_retry_policy(max_attempts: u32) -> RetryPolicy {
        RetryPolicy {
            max_attempts,
            initial_delay: Duration::from_millis(1),
            max_delay: Duration::from_millis(10),
            backoff_multiplier: 2.0,
        }
    }

    #[tokio::test]
    async fn succeeds_on_first_try() {
        let executor = RetryExecutor::new(fast_retry_policy(3));
        let result: UniversalZfsResult<&str> = executor.execute(|| async { Ok("ok") }).await;
        assert_eq!(result.unwrap(), "ok");
    }

    #[tokio::test]
    async fn retries_until_success() {
        let executor = RetryExecutor::new(fast_retry_policy(3));
        let counter = AtomicU32::new(0);
        let result: UniversalZfsResult<&str> = executor
            .execute(|| {
                let attempt = counter.fetch_add(1, Ordering::SeqCst);
                async move {
                    if attempt < 2 {
                        Result::Err(Err::internal("temporary"))
                    } else {
                        Ok("recovered")
                    }
                }
            })
            .await;
        assert_eq!(result.unwrap(), "recovered");
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn fails_after_max_attempts() {
        let executor = RetryExecutor::new(fast_retry_policy(2));
        let counter = AtomicU32::new(0);
        let result: UniversalZfsResult<()> = executor
            .execute(|| {
                counter.fetch_add(1, Ordering::SeqCst);
                async { Result::Err(Err::internal("permanent")) }
            })
            .await;
        assert!(result.is_err());
        assert_eq!(counter.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn single_attempt_does_not_retry() {
        let executor = RetryExecutor::new(fast_retry_policy(1));
        let counter = AtomicU32::new(0);
        let result: UniversalZfsResult<()> = executor
            .execute(|| {
                counter.fetch_add(1, Ordering::SeqCst);
                async { Result::Err(Err::internal("once")) }
            })
            .await;
        assert!(result.is_err());
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }
}
