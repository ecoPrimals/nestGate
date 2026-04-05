// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # Monitoring Configuration
//!
//! Monitoring and observability environment configuration extracted for logical cohesion.
//!
//! **Phase 3: Smart Refactoring** - Extracted from monolithic `environment.rs` (Jan 30, 2026)

use super::{ConfigError, Port};
use nestgate_types::{EnvSource, ProcessEnv};
use serde::{Deserialize, Serialize};
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

    /// Load from an injectable environment source (e.g. [`nestgate_types::MapEnv`] in tests).
    pub fn from_env_source(env: &dyn EnvSource) -> Result<Self, ConfigError> {
        Self::from_env_with_prefix_source("NESTGATE", env)
    }

    /// Load from environment with custom prefix
    pub fn from_env_with_prefix(prefix: &str) -> Result<Self, ConfigError> {
        Self::from_env_with_prefix_source(prefix, &ProcessEnv)
    }

    /// Load with custom prefix from an injectable [`EnvSource`].
    pub fn from_env_with_prefix_source(
        prefix: &str,
        env: &dyn EnvSource,
    ) -> Result<Self, ConfigError> {
        Ok(Self {
            metrics_port: Self::env_var_or(prefix, "METRICS_PORT", Port::new(9090)?, env)?,
            detailed_metrics: Self::env_var_or(prefix, "DETAILED_METRICS", true, env)?,
            log_level: Self::env_var_or(prefix, "LOG_LEVEL", "info".to_string(), env)?,
            tracing_enabled: Self::env_var_or(prefix, "TRACING_ENABLED", true, env)?,
            trace_sample_rate: Self::env_var_or(prefix, "TRACE_SAMPLE_RATE", 0.1, env)?,
        })
    }

    /// Helper to get environment variable or use default
    fn env_var_or<T: FromStr>(
        prefix: &str,
        key: &str,
        default: T,
        env: &dyn EnvSource,
    ) -> Result<T, ConfigError>
    where
        T::Err: std::error::Error + Send + Sync + 'static,
    {
        let var_name = format!("{prefix}_{key}");
        match env.get(&var_name) {
            Some(val) => val.parse().map_err(|e| ConfigError::ParseError {
                key: var_name,
                source: Box::new(e),
            }),
            None => Ok(default),
        }
    }
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            // 9090 is within the validated Port range (1024–65535).
            metrics_port: Port::new_unchecked(9090),
            detailed_metrics: true,
            log_level: "info".to_string(),
            tracing_enabled: true,
            trace_sample_rate: 0.1,
        }
    }
}
