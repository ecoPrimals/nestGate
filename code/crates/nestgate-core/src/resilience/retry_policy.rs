use crate::NestGateError;
//
// Provides configurable retry policies with exponential backoff for
// handling transient failures.

use crate::{Result, NestGateError};
use std::time::Duration;

/// Retry configuration
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_attempts: u32,
    /// Initial delay between retries
    pub initial_delay: Duration,
    /// Maximum delay between retries
    pub max_delay: Duration,
    /// Backoff multiplier
    pub backoff_multiplier: f64,
    /// Jitter factor to randomize delays
    pub jitter_factor: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 2.0,
            jitter_factor: 0.1,
        }
    }
}

/// Execute operation with retry policy
pub async fn execute_with_retry<F, T, E>(
    mut operation: impl FnMut() -> F,
    config: &RetryConfig,
) -> Result<T>
where
    F: Future<Output = std::result::Result<T, E>>,
    E: std::fmt::Debug,
{
    let mut current_delay = config.initial_delay;

    for attempt in 1..=config.max_attempts {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(error) => {
                if attempt == config.max_attempts {
                    return Err(NestGateError::simple(format!(
                        "Operation failed after {} attempts: {:?}",
                        config.max_attempts, error
                    )));
                }

                tracing::debug!(
                    "Retry attempt {} failed, retrying in {:?}: {:?}",
                    attempt,
                    current_delay,
                    error
                );

                // Apply jitter to delay
                let jitter = (rand::random::<f64>() - 0.5) * 2.0 * config.jitter_factor;
                let jittered_delay = Duration::from_millis(
                    (current_delay.as_millis() as f64 * (1.0 + jitter)) as u64,
                );

                tokio::time::sleep(jittered_delay).await;

                // Calculate next delay with exponential backoff
                current_delay = std::cmp::min(
                    Duration::from_millis(
                        (current_delay.as_millis() as f64 * config.backoff_multiplier) as u64,
                    ),
                    config.max_delay,
                );
            }
        }
    }

    unreachable!()
}
