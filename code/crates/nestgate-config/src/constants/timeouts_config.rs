// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Timeout configuration module
//!
//! Provides thread-safe configuration for all timeout values including connection timeouts,
//! request timeouts, idle timeouts, keepalive intervals, and retry delays. All values are
//! loaded from environment variables at initialization time.
//!
//! # Example
//!
//! ```rust,ignore
//! use nestgate_core::constants::timeouts_config::TimeoutsConfig;
//!
//! // Load from environment
//! let config = TimeoutsConfig::from_env();
//! let conn_timeout = config.connection_timeout();
//! let request_timeout = config.request_timeout();
//!
//! // Or build for testing
//! let test_config = TimeoutsConfig::from_env()
//!     .with_connection_timeout_secs(30)
//!     .with_request_timeout_secs(60);
//! ```
use std::env;
use std::sync::Arc;
use std::time::Duration;

/// Configuration for timeout values, loaded from environment variables
///
/// This struct centralizes all `env::var` calls for the `timeouts` module
/// to eliminate direct `env::var` calls from production code. All values are
/// immutable after construction, making this thread-safe.
#[derive(Debug, Clone)]
pub struct TimeoutsConfig {
    connection_timeout_secs: u64,
    request_timeout_secs: u64,
    idle_timeout_secs: u64,
    keepalive_interval_secs: u64,
    retry_delay_ms: u64,
}

/// Type alias for Sharedtimeoutsconfig
pub type SharedTimeoutsConfig = Arc<TimeoutsConfig>;

impl TimeoutsConfig {
    /// Creates a new `TimeoutsConfig` by loading values from environment variables.
    #[must_use]
    pub fn from_env() -> Self {
        use super::timeouts::{
            DEFAULT_CONNECTION_TIMEOUT_SECS, DEFAULT_IDLE_TIMEOUT_SECS, DEFAULT_KEEPALIVE_SECS,
            DEFAULT_REQUEST_TIMEOUT_SECS, DEFAULT_RETRY_DELAY_MS,
        };

        let connection_timeout_secs = env::var("NESTGATE_CONNECTION_TIMEOUT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(DEFAULT_CONNECTION_TIMEOUT_SECS);

        let request_timeout_secs = env::var("NESTGATE_REQUEST_TIMEOUT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(DEFAULT_REQUEST_TIMEOUT_SECS);

        let idle_timeout_secs = env::var("NESTGATE_IDLE_TIMEOUT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(DEFAULT_IDLE_TIMEOUT_SECS);

        let keepalive_interval_secs = env::var("NESTGATE_KEEPALIVE_INTERVAL")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(DEFAULT_KEEPALIVE_SECS);

        let retry_delay_ms = env::var("NESTGATE_RETRY_DELAY_MS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(DEFAULT_RETRY_DELAY_MS);

        Self {
            connection_timeout_secs,
            request_timeout_secs,
            idle_timeout_secs,
            keepalive_interval_secs,
            retry_delay_ms,
        }
    }

    // Getter methods

    /// Connection Timeout
    #[must_use]
    pub const fn connection_timeout(&self) -> Duration {
        Duration::from_secs(self.connection_timeout_secs)
    }

    /// Request Timeout
    #[must_use]
    pub const fn request_timeout(&self) -> Duration {
        Duration::from_secs(self.request_timeout_secs)
    }

    /// Idle Timeout
    #[must_use]
    pub const fn idle_timeout(&self) -> Duration {
        Duration::from_secs(self.idle_timeout_secs)
    }

    /// Keepalive Interval
    #[must_use]
    pub const fn keepalive_interval(&self) -> Duration {
        Duration::from_secs(self.keepalive_interval_secs)
    }

    /// Retry Delay
    #[must_use]
    pub const fn retry_delay(&self) -> Duration {
        Duration::from_millis(self.retry_delay_ms)
    }

    // Builder methods for testing

    /// Builder method to set Connection Timeout Secs
    #[must_use]
    pub const fn with_connection_timeout_secs(mut self, secs: u64) -> Self {
        self.connection_timeout_secs = secs;
        self
    }

    /// Builder method to set Request Timeout Secs
    #[must_use]
    pub const fn with_request_timeout_secs(mut self, secs: u64) -> Self {
        self.request_timeout_secs = secs;
        self
    }

    /// Builder method to set Idle Timeout Secs
    #[must_use]
    pub const fn with_idle_timeout_secs(mut self, secs: u64) -> Self {
        self.idle_timeout_secs = secs;
        self
    }

    /// Builder method to set Keepalive Interval Secs
    #[must_use]
    pub const fn with_keepalive_interval_secs(mut self, secs: u64) -> Self {
        self.keepalive_interval_secs = secs;
        self
    }

    /// Builder method to set Retry Delay Ms
    #[must_use]
    pub const fn with_retry_delay_ms(mut self, ms: u64) -> Self {
        self.retry_delay_ms = ms;
        self
    }
}

impl Default for TimeoutsConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self::from_env()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        use super::super::timeouts::*;
        let config = TimeoutsConfig::from_env();
        // Should use constants as defaults
        assert!(config.connection_timeout().as_secs() >= DEFAULT_CONNECTION_TIMEOUT_SECS);
    }

    #[test]
    fn test_builder_pattern() {
        let config = TimeoutsConfig::from_env()
            .with_connection_timeout_secs(45)
            .with_request_timeout_secs(120)
            .with_idle_timeout_secs(600)
            .with_keepalive_interval_secs(30)
            .with_retry_delay_ms(500);

        assert_eq!(config.connection_timeout(), Duration::from_secs(45));
        assert_eq!(config.request_timeout(), Duration::from_secs(120));
        assert_eq!(config.idle_timeout(), Duration::from_secs(600));
        assert_eq!(config.keepalive_interval(), Duration::from_secs(30));
        assert_eq!(config.retry_delay(), Duration::from_millis(500));
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 8)]
    async fn test_concurrent_access() {
        let config = Arc::new(
            TimeoutsConfig::from_env()
                .with_connection_timeout_secs(10)
                .with_request_timeout_secs(20),
        );

        let handles: Vec<_> = (0..100)
            .map(|_| {
                let cfg = config.clone();
                tokio::spawn(async move {
                    let _ = cfg.connection_timeout();
                    let _ = cfg.request_timeout();
                    let _ = cfg.idle_timeout();
                    let _ = cfg.keepalive_interval();
                    let _ = cfg.retry_delay();
                })
            })
            .collect();

        for handle in handles {
            handle.await.expect("Task should complete successfully");
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 8)]
    async fn test_concurrent_different_configs() {
        let config1 = Arc::new(
            TimeoutsConfig::from_env()
                .with_connection_timeout_secs(10)
                .with_request_timeout_secs(20),
        );
        let config2 = Arc::new(
            TimeoutsConfig::from_env()
                .with_connection_timeout_secs(30)
                .with_request_timeout_secs(60),
        );

        let handle1 = tokio::spawn({
            let cfg = config1.clone();
            async move { (cfg.connection_timeout(), cfg.request_timeout()) }
        });
        let handle2 = tokio::spawn({
            let cfg = config2.clone();
            async move { (cfg.connection_timeout(), cfg.request_timeout()) }
        });

        let (timeout1_conn, timeout1_req) = handle1.await.unwrap();
        let (timeout2_conn, timeout2_req) = handle2.await.unwrap();

        assert_eq!(timeout1_conn, Duration::from_secs(10));
        assert_eq!(timeout1_req, Duration::from_secs(20));
        assert_eq!(timeout2_conn, Duration::from_secs(30));
        assert_eq!(timeout2_req, Duration::from_secs(60));
    }
}
