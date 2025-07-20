//! Retry Executor Implementation
//!
//! Provides retry logic for fail-safe operations.

use std::time::Duration;
use tracing::debug;

use crate::handlers::zfs::universal_zfs::config::RetryPolicy;
use crate::handlers::zfs::universal_zfs::types::UniversalZfsResult;

/// Retry executor for operations
#[derive(Debug, Clone)]
pub struct RetryExecutor {
    config: RetryPolicy,
}

impl RetryExecutor {
    pub fn new(config: RetryPolicy) -> Self {
        Self { config }
    }

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
