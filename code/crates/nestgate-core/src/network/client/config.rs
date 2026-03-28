// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! HTTP Client Configuration
//!
//! Type-safe configuration for HTTP client with sensible defaults
//! and const generic compile-time parameters.

use serde::{Deserialize, Serialize};
use std::time::Duration;

// ==================== CLIENT CONFIG ====================

/// HTTP client configuration with const generic defaults
///
/// Uses const generics to provide compile-time configuration where possible.
/// Default timeout is 30 seconds (30,000 ms).
///
/// # Examples
/// ```ignore
/// // Default configuration (30s timeout)
/// let config = ClientConfig::default();
///
/// // Custom timeout (60s)
/// let config = ClientConfig::<60000>::default();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientConfig<const DEFAULT_TIMEOUT_MS: u64 = 30000> {
    /// Request timeout
    pub timeout: Duration,
    /// Maximum number of connections per host
    pub max_connections_per_host: usize,
    /// Maximum number of idle connections
    pub max_idle_connections: usize,
    /// Idle connection timeout
    pub idle_timeout: Duration,
    /// Number of retry attempts
    pub max_retries: usize,
    /// Initial retry delay
    pub retry_delay: Duration,
    /// Enable response compression (gzip, deflate)
    pub enable_compression: bool,
    /// Follow HTTP redirects automatically
    pub follow_redirects: bool,
}

impl<const DEFAULT_TIMEOUT_MS: u64> ClientConfig<DEFAULT_TIMEOUT_MS> {
    /// Create a new configuration
    pub fn new() -> Self {
        Self {
            timeout: Duration::from_millis(DEFAULT_TIMEOUT_MS),
            max_connections_per_host: 10,
            max_idle_connections: 100,
            idle_timeout: Duration::from_secs(90),
            max_retries: 3,
            retry_delay: Duration::from_millis(100),
            enable_compression: true,
            follow_redirects: true,
        }
    }

    /// Set request timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set maximum connections per host
    pub fn with_max_connections(mut self, max: usize) -> Self {
        self.max_connections_per_host = max;
        self
    }

    /// Set maximum retries
    pub fn with_max_retries(mut self, max: usize) -> Self {
        self.max_retries = max;
        self
    }

    /// Enable or disable compression
    pub fn with_compression(mut self, enabled: bool) -> Self {
        self.enable_compression = enabled;
        self
    }

    /// Enable or disable automatic redirects
    pub fn with_redirects(mut self, enabled: bool) -> Self {
        self.follow_redirects = enabled;
        self
    }
}

impl<const DEFAULT_TIMEOUT_MS: u64> Default for ClientConfig<DEFAULT_TIMEOUT_MS> {
    fn default() -> Self {
        Self::new()
    }
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ClientConfig::<30000>::default();
        assert_eq!(config.timeout, Duration::from_millis(30000));
        assert_eq!(config.max_connections_per_host, 10);
        assert_eq!(config.max_retries, 3);
    }

    #[test]
    fn test_custom_timeout() {
        let config = ClientConfig::<60000>::default();
        assert_eq!(config.timeout, Duration::from_millis(60000));
    }

    #[test]
    fn test_with_methods() {
        let config = ClientConfig::<30000>::default()
            .with_timeout(Duration::from_secs(45))
            .with_max_connections(20)
            .with_max_retries(5);

        assert_eq!(config.timeout, Duration::from_secs(45));
        assert_eq!(config.max_connections_per_host, 20);
        assert_eq!(config.max_retries, 5);
    }
}
