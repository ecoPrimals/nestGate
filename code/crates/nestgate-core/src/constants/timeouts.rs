//! Timeout and duration constants
//!
//! Centralized timeout configuration with environment variable support.

use std::env;
use std::time::Duration;

/// Default connection timeout in seconds
pub const DEFAULT_CONNECTION_TIMEOUT_SECS: u64 = 30;

/// Default request timeout in seconds
pub const DEFAULT_REQUEST_TIMEOUT_SECS: u64 = 60;

/// Default idle timeout in seconds
pub const DEFAULT_IDLE_TIMEOUT_SECS: u64 = 300;

/// Default keepalive interval in seconds
pub const DEFAULT_KEEPALIVE_SECS: u64 = 60;

/// Default retry delay in milliseconds
pub const DEFAULT_RETRY_DELAY_MS: u64 = 1000;

/// Default health check interval in seconds
pub const DEFAULT_HEALTH_CHECK_INTERVAL_SECS: u64 = 30;

/// Get connection timeout from environment or use default
///
/// Environment variable: `NESTGATE_CONNECTION_TIMEOUT`
/// Default: `30` seconds
pub fn connection_timeout() -> Duration {
    let secs = env::var("NESTGATE_CONNECTION_TIMEOUT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_CONNECTION_TIMEOUT_SECS);
    Duration::from_secs(secs)
}

/// Get request timeout from environment or use default
///
/// Environment variable: `NESTGATE_REQUEST_TIMEOUT`
/// Default: `60` seconds
pub fn request_timeout() -> Duration {
    let secs = env::var("NESTGATE_REQUEST_TIMEOUT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_REQUEST_TIMEOUT_SECS);
    Duration::from_secs(secs)
}

/// Get idle timeout from environment or use default
///
/// Environment variable: `NESTGATE_IDLE_TIMEOUT`
/// Default: `300` seconds (5 minutes)
pub fn idle_timeout() -> Duration {
    let secs = env::var("NESTGATE_IDLE_TIMEOUT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_IDLE_TIMEOUT_SECS);
    Duration::from_secs(secs)
}

/// Get keepalive interval from environment or use default
///
/// Environment variable: `NESTGATE_KEEPALIVE_INTERVAL`
/// Default: `60` seconds
pub fn keepalive_interval() -> Duration {
    let secs = env::var("NESTGATE_KEEPALIVE_INTERVAL")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_KEEPALIVE_SECS);
    Duration::from_secs(secs)
}

/// Get retry delay from environment or use default
///
/// Environment variable: `NESTGATE_RETRY_DELAY_MS`
/// Default: `1000` milliseconds (1 second)
pub fn retry_delay() -> Duration {
    let ms = env::var("NESTGATE_RETRY_DELAY_MS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_RETRY_DELAY_MS);
    Duration::from_millis(ms)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_timeouts() {
        assert_eq!(DEFAULT_CONNECTION_TIMEOUT_SECS, 30);
        assert_eq!(DEFAULT_REQUEST_TIMEOUT_SECS, 60);
        assert_eq!(DEFAULT_IDLE_TIMEOUT_SECS, 300);
    }

    #[test]
    fn test_connection_timeout_default() {
        env::remove_var("NESTGATE_CONNECTION_TIMEOUT");
        assert_eq!(connection_timeout(), Duration::from_secs(30));
    }

    #[test]
    fn test_connection_timeout_from_env() {
        env::set_var("NESTGATE_CONNECTION_TIMEOUT", "45");
        assert_eq!(connection_timeout(), Duration::from_secs(45));
        env::remove_var("NESTGATE_CONNECTION_TIMEOUT");
    }
}

