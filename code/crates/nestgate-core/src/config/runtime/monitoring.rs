//! Monitoring configuration module
//!
//! Provides configuration for metrics, logging, and tracing.

use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::env;

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
        Ok(Self {
            metrics_enabled: env::var("NESTGATE_METRICS_ENABLED")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(true),
            metrics_port: env::var("NESTGATE_METRICS_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(9090),
            log_level: env::var("NESTGATE_LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
            tracing_enabled: env::var("NESTGATE_TRACING_ENABLED")
                .ok()
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
