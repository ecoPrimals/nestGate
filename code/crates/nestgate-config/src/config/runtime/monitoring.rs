// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Monitoring configuration module
//!
//! Provides configuration for metrics, logging, and tracing.

use nestgate_types::error::Result;
use nestgate_types::{EnvSource, ProcessEnv};
use serde::{Deserialize, Serialize};

/// Monitoring configuration for observability.
///
/// # Environment Variables
///
/// - `NESTGATE_METRICS_ENABLED` - Enable metrics (default: true)
/// - `NESTGATE_METRICS_PORT` - Metrics port (default: 9090)
/// - `NESTGATE_LOG_LEVEL` - Log level (default: "info")
/// - `NESTGATE_TRACING_ENABLED` - Enable tracing (default: false)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Enable metrics collection
    pub metrics_enabled: bool,

    /// Metrics port
    pub metrics_port: u16,

    /// Log level (trace, debug, info, warn, error)
    pub log_level: String,

    /// Enable distributed tracing
    pub tracing_enabled: bool,
}

impl MonitoringConfig {
    /// Load monitoring configuration from environment variables.
    pub fn from_environment() -> Result<Self> {
        Self::from_env_source(&ProcessEnv)
    }

    /// Like [`Self::from_environment`], but reads monitoring variables from `env`.
    pub fn from_env_source(env: &dyn EnvSource) -> Result<Self> {
        Ok(Self {
            metrics_enabled: env
                .get("NESTGATE_METRICS_ENABLED")
                .and_then(|s| s.parse().ok())
                .unwrap_or(true),
            metrics_port: env
                .get("NESTGATE_METRICS_PORT")
                .and_then(|s| s.parse().ok())
                .unwrap_or(9090),
            log_level: env
                .get("NESTGATE_LOG_LEVEL")
                .unwrap_or_else(|| "info".to_string()),
            tracing_enabled: env
                .get("NESTGATE_TRACING_ENABLED")
                .and_then(|s| s.parse().ok())
                .unwrap_or(false),
        })
    }
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            metrics_enabled: true,
            metrics_port: 9090,
            log_level: "info".to_string(),
            tracing_enabled: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nestgate_types::MapEnv;

    #[test]
    fn test_monitoring_config_default() {
        let config = MonitoringConfig::default();
        assert!(config.metrics_enabled);
        assert_eq!(config.metrics_port, 9090);
        assert_eq!(config.log_level, "info");
        assert!(!config.tracing_enabled);
    }

    #[test]
    fn test_monitoring_config_from_environment_defaults() {
        let env = MapEnv::new();
        let config = MonitoringConfig::from_env_source(&env).unwrap();
        assert!(config.metrics_enabled);
        assert_eq!(config.metrics_port, 9090);
        assert_eq!(config.log_level, "info");
        assert!(!config.tracing_enabled);
    }

    #[test]
    fn test_monitoring_config_from_environment_overrides() {
        let env = MapEnv::from([
            ("NESTGATE_METRICS_ENABLED", "false"),
            ("NESTGATE_METRICS_PORT", "9091"),
            ("NESTGATE_LOG_LEVEL", "debug"),
            ("NESTGATE_TRACING_ENABLED", "true"),
        ]);
        let config = MonitoringConfig::from_env_source(&env).unwrap();
        assert!(!config.metrics_enabled);
        assert_eq!(config.metrics_port, 9091);
        assert_eq!(config.log_level, "debug");
        assert!(config.tracing_enabled);
    }

    #[test]
    fn test_monitoring_config_serialization() {
        let config = MonitoringConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("info"));
        let parsed: MonitoringConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.metrics_port, 9090);
    }
}
