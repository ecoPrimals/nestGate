// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Monitoring environment configuration module
//!
//! Provides configuration for monitoring and observability features including
//! log rotation, metrics collection, and tracing. All values are loaded from
//! environment variables with sensible defaults.
//!
//! # Example
//!
//! ```rust,ignore
//! use nestgate_core::config::monitoring_env_config::MonitoringEnvConfig;
//!
//! // Load from environment
//! let config = MonitoringEnvConfig::from_env();
//! let rotation_size = config.log_rotation_size_bytes();
//!
//! // Or build for testing
//! let test_config = MonitoringEnvConfig::from_env()
//!     .with_log_rotation_size_bytes(5 * 1024 * 1024); // 5MB
//! ```
//!
//! # Environment Variables
//!
//! - `NESTGATE_LOG_ROTATION_SIZE_BYTES`: Log file rotation size (default: 1MB)
use std::env;
use std::sync::Arc;

/// Configuration for monitoring module, loaded from environment variables
///
/// Manages observability settings including log rotation policies,
/// metrics collection parameters, and tracing configuration.
#[derive(Debug, Clone)]
pub struct MonitoringEnvConfig {
    log_rotation_size_bytes: usize,
}

/// Type alias for Sharedmonitoringenvconfig
pub type SharedMonitoringEnvConfig = Arc<MonitoringEnvConfig>;

impl MonitoringEnvConfig {
    /// Creates a new `MonitoringEnvConfig` by loading values from environment variables.
    #[must_use]
    pub fn from_env() -> Self {
        let log_rotation_size_bytes = env::var("NESTGATE_LOG_ROTATION_SIZE_BYTES")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1024 * 1024); // 1MB default

        Self {
            log_rotation_size_bytes,
        }
    }

    /// Log Rotation Size Bytes
    #[must_use]
    pub const fn log_rotation_size_bytes(&self) -> usize {
        self.log_rotation_size_bytes
    }

    /// Builder method to set log rotation size in bytes
    ///
    /// # Arguments
    ///
    /// * `size` - Log file size in bytes before rotation
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// # use nestgate_core::config::monitoring_env_config::MonitoringEnvConfig;
    /// let config = MonitoringEnvConfig::from_env()
    ///     .with_log_rotation_size_bytes(10 * 1024 * 1024); // 10MB
    /// ```
    #[must_use]
    pub const fn with_log_rotation_size_bytes(mut self, size: usize) -> Self {
        self.log_rotation_size_bytes = size;
        self
    }
}

impl Default for MonitoringEnvConfig {
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
        let config = MonitoringEnvConfig::from_env();
        assert_eq!(config.log_rotation_size_bytes(), 1024 * 1024);
    }

    #[test]
    fn test_builder() {
        let config = MonitoringEnvConfig::from_env().with_log_rotation_size_bytes(2048 * 1024);
        assert_eq!(config.log_rotation_size_bytes(), 2048 * 1024);
    }
}
