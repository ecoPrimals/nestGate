// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! # Monitoring Configuration
//!
//! Monitoring and observability environment configuration extracted for logical cohesion.
//!
//! **Phase 3: Smart Refactoring** - Extracted from monolithic `environment.rs` (Jan 30, 2026)

use super::{ConfigError, Port};
use serde::{Deserialize, Serialize};
use std::env;
use std::str::FromStr;

/// Monitoring and observability configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Metrics port (default: 9090)
    pub metrics_port: Port,

    /// Enable detailed metrics (default: true)
    pub detailed_metrics: bool,

    /// Log level (default: info)
    pub log_level: String,

    /// Enable tracing (default: true)
    pub tracing_enabled: bool,

    /// Trace sample rate (0.0-1.0, default: 0.1)
    pub trace_sample_rate: f64,
}

impl MonitoringConfig {
    /// Load from environment
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::from_env_with_prefix("NESTGATE")
    }

    /// Load from environment with custom prefix
    pub fn from_env_with_prefix(prefix: &str) -> Result<Self, ConfigError> {
        Ok(Self {
            metrics_port: Self::env_var_or(prefix, "METRICS_PORT", Port::new(9090)?)?,
            detailed_metrics: Self::env_var_or(prefix, "DETAILED_METRICS", true)?,
            log_level: Self::env_var_or(prefix, "LOG_LEVEL", "info".to_string())?,
            tracing_enabled: Self::env_var_or(prefix, "TRACING_ENABLED", true)?,
            trace_sample_rate: Self::env_var_or(prefix, "TRACE_SAMPLE_RATE", 0.1)?,
        })
    }

    /// Helper to get environment variable or use default
    fn env_var_or<T: FromStr>(prefix: &str, key: &str, default: T) -> Result<T, ConfigError>
    where
        T::Err: std::error::Error + Send + Sync + 'static,
    {
        let var_name = format!("{}_{}", prefix, key);
        match env::var(&var_name) {
            Ok(val) => val.parse().map_err(|e| ConfigError::ParseError {
                key: var_name,
                source: Box::new(e),
            }),
            Err(_) => Ok(default),
        }
    }
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            metrics_port: Port::new(9090).expect("Default metrics port"),
            detailed_metrics: true,
            log_level: "info".to_string(),
            tracing_enabled: true,
            trace_sample_rate: 0.1,
        }
    }
}
