// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Canonical Configuration System for `NestGate`,
//!
//! Unified configuration that replaces all fragmented config structs
//! across the ecosystem with a single, canonical configuration hierarchy.

use std::path::Path;

use crate::error::{NestGateError, Result};
use crate::types::CanonicalConfig;

/// Canonical `NestGate` Configuration,
///
/// This replaces all fragmented configuration structs with a unified system
pub type NestGateConfig = CanonicalConfig;
impl NestGateConfig {
    /// Load configuration from environment variables
    /// Creates a canonical configuration from environment variables.
    ///
    /// Currently returns the default configuration but will be extended
    /// to read from environment variables in future versions.
    #[must_use]
    pub fn from_env() -> Self {
        Self::default()
    }

    /// Load configuration from file
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let content = std::fs::read_to_string(path.as_ref())
            .map_err(|e| NestGateError::validation(format!("Failed to read config file: {e}")))?;

        let config: Self = toml::from_str(&content)
            .map_err(|e| NestGateError::validation(format!("Failed to parse config: {e}")))?;

        config.validate()?;
        Ok(config)
    }

    /// Validate the configuration
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    #[allow(deprecated)] // MIGRATION: Use CanonicalNetworkConfig in next major version
    pub fn validate(&self) -> Result<()> {
        if self.service_name.is_empty() {
            return Err(NestGateError::validation("Service name cannot be empty"));
        }

        if self.network.port == 0 {
            return Err(NestGateError::validation("Port cannot be zero"));
        }

        if self.storage.data_directory.is_empty() {
            return Err(NestGateError::validation("Data directory cannot be empty"));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nestgate_config_from_env() {
        let config = NestGateConfig::from_env();
        assert_eq!(config.service_name, "nestgate");
        assert_eq!(config.version, "2.0.0");
        assert_eq!(config.environment, "production");
        assert!(!config.debug_mode);
        assert_eq!(config.log_level, "info");
    }

    #[test]
    fn test_nestgate_config_validate_success() {
        let config = NestGateConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_nestgate_config_validate_empty_service_name() {
        let config = NestGateConfig {
            service_name: String::new(),
            ..Default::default()
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Service name"));
    }

    #[test]
    #[allow(deprecated)]
    fn test_nestgate_config_validate_zero_port() {
        let mut config = NestGateConfig::default();
        config.network.port = 0;
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Port"));
    }

    #[test]
    fn test_nestgate_config_validate_empty_data_dir() {
        let mut config = NestGateConfig::default();
        config.storage.data_directory = String::new();
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Data directory"));
    }

    #[test]
    fn test_nestgate_config_from_file_nonexistent() {
        let result = NestGateConfig::from_file("/nonexistent/path/config.toml");
        assert!(result.is_err());
    }

    #[test]
    #[allow(deprecated)] // Testing deprecated SecurityConfig fields
    fn test_nestgate_config_default_values() {
        let config = NestGateConfig::default();
        assert_eq!(config.service_name, "nestgate");
        assert_eq!(config.version, "2.0.0");
        assert_eq!(config.environment, "production");
        assert!(!config.debug_mode);
        assert_eq!(config.log_level, "info");
        assert_eq!(config.storage.cache_size_mb, 512);
        assert!(config.storage.compression_enabled);
        assert_eq!(config.security.session_timeout_minutes, 60);
    }

    #[test]
    #[allow(deprecated)]
    fn test_nestgate_config_default_network() {
        let config = NestGateConfig::default();
        assert_eq!(config.network.timeout_seconds, 30);
        assert_eq!(config.network.max_connections, 1000);
        assert!(!config.network.enable_tls);
    }

    #[test]
    fn test_nestgate_config_default_storage() {
        let config = NestGateConfig::default();
        assert_eq!(config.storage.backend_type, "local");
        assert!(config.storage.compression_enabled);
        assert!(!config.storage.encryption_enabled);
        assert!(config.storage.backup_enabled);
    }

    #[test]
    #[allow(deprecated)] // Testing deprecated SecurityConfig fields
    fn test_nestgate_config_default_security() {
        let config = NestGateConfig::default();
        assert!(config.security.authentication_enabled);
        assert!(config.security.authorization_enabled);
        assert_eq!(config.security.max_login_attempts, 5);
    }

    #[test]
    fn test_nestgate_config_default_performance() {
        let config = NestGateConfig::default();
        assert!(config.performance.thread_pool_size > 0);
        assert_eq!(config.performance.buffer_size_kb, 1024);
        assert_eq!(config.performance.batch_size, 100);
        assert!(config.performance.enable_metrics);
    }
}
