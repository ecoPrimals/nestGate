//
// Provides retry logic for fail-safe operations.

use std::time::Duration;
// Removed unused tracing import

use crate::handlers::zfs::universal_zfs::config::RetryPolicy;
use crate::handlers::zfs::universal_zfs::types::UniversalZfsResult;
use tracing::debug;

/// Retry executor for operations
#[derive(Debug, Clone)]
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
    /// * `F` - Function type that returns a Future
    /// * `T` - Return type of the operation
    pub async fn execute<F, T>(&self, operation: F) -> UniversalZfsResult<T>
    where
        F: Fn() -> std::pin::Pin<
            Box<dyn std::future::Future<Output = UniversalZfsResult<T>> + Send + 'static>,
        >,
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
