// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Timeout and duration constants
//!
//! Centralized timeout configuration with environment variable support.

use std::time::Duration;

// Import config for environment variable lookups
use super::timeouts_config::TimeoutsConfig;

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
#[must_use]
pub fn connection_timeout() -> Duration {
    TimeoutsConfig::from_env().connection_timeout()
}

/// Get request timeout from environment or use default
///
/// Environment variable: `NESTGATE_REQUEST_TIMEOUT`
/// Default: `60` seconds
#[must_use]
pub fn request_timeout() -> Duration {
    TimeoutsConfig::from_env().request_timeout()
}

/// Get idle timeout from environment or use default
///
/// Environment variable: `NESTGATE_IDLE_TIMEOUT`
/// Default: `300` seconds (5 minutes)
#[must_use]
pub fn idle_timeout() -> Duration {
    TimeoutsConfig::from_env().idle_timeout()
}

/// Get keepalive interval from environment or use default
///
/// Environment variable: `NESTGATE_KEEPALIVE_INTERVAL`
/// Default: `60` seconds
#[must_use]
pub fn keepalive_interval() -> Duration {
    TimeoutsConfig::from_env().keepalive_interval()
}

/// Get retry delay from environment or use default
///
/// Environment variable: `NESTGATE_RETRY_DELAY_MS`
/// Default: `1000` milliseconds (1 second)
#[must_use]
pub fn retry_delay() -> Duration {
    TimeoutsConfig::from_env().retry_delay()
}

#[cfg(test)]
mod tests {
    use super::super::timeouts_config::TimeoutsConfig;
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_default_timeouts() {
        assert_eq!(DEFAULT_CONNECTION_TIMEOUT_SECS, 30);
        assert_eq!(DEFAULT_REQUEST_TIMEOUT_SECS, 60);
        assert_eq!(DEFAULT_IDLE_TIMEOUT_SECS, 300);
    }

    #[test]
    fn test_connection_timeout_default() {
        // Test with config using defaults
        let config = TimeoutsConfig::from_env();
        assert!(config.connection_timeout().as_secs() >= 30);
    }

    #[test]
    fn test_connection_timeout_with_config() {
        // Test with injected config
        let config = Arc::new(TimeoutsConfig::from_env().with_connection_timeout_secs(45));
        assert_eq!(config.connection_timeout(), Duration::from_secs(45));
    }

    #[test]
    fn test_all_timeouts_with_config() {
        let config = Arc::new(
            TimeoutsConfig::from_env()
                .with_connection_timeout_secs(10)
                .with_request_timeout_secs(20)
                .with_idle_timeout_secs(300)
                .with_keepalive_interval_secs(30)
                .with_retry_delay_ms(500),
        );

        assert_eq!(config.connection_timeout(), Duration::from_secs(10));
        assert_eq!(config.request_timeout(), Duration::from_secs(20));
        assert_eq!(config.idle_timeout(), Duration::from_secs(300));
        assert_eq!(config.keepalive_interval(), Duration::from_secs(30));
        assert_eq!(config.retry_delay(), Duration::from_millis(500));
    }
}
