// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(clippy::wildcard_imports)]

//! Monitoring configuration — alerting, logging, metrics, tracing.
// Removed unused error imports

use serde::{Deserialize, Serialize};

mod alerting;
mod constants;
mod metrics_config;
mod notifications;

pub use alerting::{AlertConfig, AlertThresholds};
pub use metrics_config::PrometheusConfig;
pub use notifications::{EmailConfig, NotificationConfig, SlackConfig, WebhookConfig};

use self::constants::*;

/// Monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Monitoring
pub struct MonitoringConfig {
    /// Metrics collection interval in seconds
    pub metrics_interval: u64,
    /// Log level for monitoring
    pub log_level: String,

    /// Log file path
    pub log_file: String,

    /// Log rotation size in bytes
    pub log_rotation_size: u64,

    /// Log retention in days
    pub log_retention_days: u32,

    /// Prometheus configuration
    pub prometheus: Option<PrometheusConfig>,

    /// Alert configuration
    pub alerts: AlertConfig,
}

impl Default for MonitoringConfig {
    /// Returns the default instance
    fn default() -> Self {
        use super::monitoring_env_config::MonitoringEnvConfig;
        let env_config = MonitoringEnvConfig::from_env();

        Self {
            metrics_interval: 30,
            log_level: "info".to_string(),
            log_file: LOG_FILE_DEFAULT.to_string(),
            log_rotation_size: env_config.log_rotation_size_bytes() as u64,
            log_retention_days: 30,
            prometheus: Some(PrometheusConfig::default()),
            alerts: AlertConfig::default(),
        }
    }
}

impl MonitoringConfig {
    /// Check if Prometheus is enabled
    #[must_use]
    pub fn is_prometheus_enabled(&self) -> bool {
        self.prometheus.as_ref().is_some_and(|p| p.enabled)
    }

    /// Check if alerting is enabled
    #[must_use]
    pub const fn is_alerting_enabled(&self) -> bool {
        self.alerts.enabled
    }

    /// Get Prometheus port if enabled
    #[must_use]
    pub fn prometheus_port(&self) -> Option<u16> {
        self.prometheus.as_ref().and_then(|p| {
            if p.enabled && p.port > 0 {
                Some(p.port)
            } else {
                None
            }
        })
    }

    /// Validate monitoring configuration
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn validate(&self) -> Result<(), &'static str> {
        // Validate metrics interval
        if self.metrics_interval == 0 {
            return Err(ERROR_METRICS_INTERVAL_ZERO);
        }

        // Validate log file path
        if self.log_file.is_empty() {
            return Err(ERROR_LOG_FILE_EMPTY);
        }

        // Validate log rotation size
        if self.log_rotation_size == 0 {
            return Err(ERROR_LOG_ROTATION_SIZE_ZERO);
        }

        // Validate log retention days
        if self.log_retention_days == 0 {
            return Err(ERROR_LOG_RETENTION_ZERO);
        }

        // Validate Prometheus configuration
        if let Some(prometheus) = &self.prometheus
            && prometheus.enabled
            && prometheus.port == 0
        {
            return Err(ERROR_PROMETHEUS_PORT_ZERO);
        }

        // Validate alert configuration
        if self.alerts.enabled {
            self.alerts.validate()?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
