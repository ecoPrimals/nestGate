// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Prometheus scrape endpoint and metrics-port configuration.

use serde::{Deserialize, Serialize};

/// Prometheus configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Prometheus
pub struct PrometheusConfig {
    /// Enable Prometheus metrics
    pub enabled: bool,
    /// Prometheus metrics port
    pub port: u16,
}

impl Default for PrometheusConfig {
    /// Returns the default instance
    ///
    /// Loads Prometheus configuration from environment:
    /// - `NESTGATE_METRICS_PORT`: Prometheus metrics port (default: 9090)
    fn default() -> Self {
        use crate::config::environment::EnvironmentConfig;

        let env_config =
            EnvironmentConfig::from_env().unwrap_or_else(|_| EnvironmentConfig::default());

        Self {
            enabled: true,
            port: env_config.monitoring.metrics_port.get(), // Standard Prometheus port
        }
    }
}
