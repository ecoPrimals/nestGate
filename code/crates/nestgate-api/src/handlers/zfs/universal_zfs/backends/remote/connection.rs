use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Connection statistics for monitoring
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConnectionStats {
    /// Total number of requests made through this connection
    pub total_requests: u64,
    /// Number of requests that completed successfully
    pub successful_requests: u64,
    /// Number of requests that failed
    pub failed_requests: u64,
    /// Average response time for all requests
    pub average_response_time: Duration,
    /// Last error message if any recent failure occurred
    pub last_error: Option<String>,
    /// Number of consecutive failures (resets on success)
    pub consecutive_failures: u32,
}
impl ConnectionStats {
    /// Create new connection stats
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a successful request
    pub const fn record_success(&mut self, response_time: Duration) {
        self.total_requests += 1;
        self.successful_requests += 1;
        self.consecutive_failures = 0;

        // Update average response time (simple moving average)
        let total_time = self.average_response_time.as_millis() as u64
            * (self.successful_requests - 1)
            + response_time.as_millis() as u64;
        self.average_response_time = Duration::from_millis(total_time / self.successful_requests);
    }

    /// Record a failed request
    pub fn record_failure(&mut self, error: String) {
        self.total_requests += 1;
        self.failed_requests += 1;
        self.consecutive_failures += 1;
        self.last_error = Some(error);
    }

    /// Get success rate as percentage
    #[must_use]
    pub fn success_rate(&self) -> f64 {
        if self.total_requests == 0 {
            return 100.0;
        }
        (self.successful_requests as f64 / self.total_requests as f64) * 100.0
    }

    /// Check if connection is healthy
    #[must_use]
    pub fn is_healthy(&self) -> bool {
        self.consecutive_failures < 3 && self.success_rate() > 80.0
    }
}

/// Connection-related errors
#[derive(Debug, thiserror::Error)]
pub enum ConnectionError {
    /// Connection timeout occurred
    #[error("Connection timeout: {0}")]
    Timeout(String),
    /// Connection was refused by the remote service
    #[error("Connection refused: {0}")]
    Refused(String),

    /// Network-level error occurred
    #[error("Network error: {0}")]
    Network(String),

    /// Authentication with remote service failed
    #[error("Authentication failed: {0}")]
    Auth(String),

    /// Too many consecutive failures, connection disabled
    #[error("Too many failures: {consecutive} consecutive failures")]
    TooManyFailures {
        /// Number of consecutive failures that triggered this error
        consecutive: u32,
    },
}
