// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Alert thresholds and top-level alert configuration validation.

#![allow(clippy::wildcard_imports)]

use serde::{Deserialize, Serialize};

use super::constants::*;
use super::notifications::NotificationConfig;

/// Alert configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for Alert
pub struct AlertConfig {
    /// Enable alerting
    pub enabled: bool,
    /// Alert thresholds
    pub thresholds: AlertThresholds,

    /// Notification configuration
    pub notifications: NotificationConfig,
}

/// Alert thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Alertthresholds
pub struct AlertThresholds {
    /// CPU usage threshold (percentage)
    pub cpu_threshold: f64,
    /// Memory usage threshold (percentage)
    pub memory_threshold: f64,

    /// Disk usage threshold (percentage)
    pub disk_threshold: f64,

    /// Latency threshold (milliseconds)
    pub latency_threshold: f64,

    /// Error rate threshold (percentage)
    pub error_rate_threshold: f64,
}

impl Default for AlertThresholds {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            cpu_threshold: 80.0,
            memory_threshold: 85.0,
            disk_threshold: 90.0,
            latency_threshold: 1000.0,
            error_rate_threshold: 5.0,
        }
    }
}

impl AlertConfig {
    /// Check if any notification method is configured
    #[must_use]
    pub const fn has_notifications(&self) -> bool {
        self.notifications.email.is_some()
            || self.notifications.slack.is_some()
            || self.notifications.webhook.is_some()
    }

    /// Validate alert configuration
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn validate(&self) -> Result<(), &'static str> {
        // Validate thresholds
        self.thresholds.validate()?;

        // Validate notification configuration
        if self.enabled && !self.has_notifications() {
            return Err(ERROR_NOTIFICATION_REQUIRED);
        }

        self.notifications.validate()?;
        Ok(())
    }
}

impl AlertThresholds {
    /// Check if a threshold is exceeded
    #[must_use]
    pub fn is_threshold_exceeded(&self, metric: &str, value: f64) -> bool {
        match metric {
            "cpu" => value > self.cpu_threshold,
            "memory" => value > self.memory_threshold,
            "disk" => value > self.disk_threshold,
            "latency" => value > self.latency_threshold,
            "error_rate" => value > self.error_rate_threshold,
            _ => false,
        }
    }

    /// Get threshold value for a metric
    #[must_use]
    pub fn get_threshold(&self, metric: &str) -> Option<f64> {
        match metric {
            "cpu" => Some(self.cpu_threshold),
            "memory" => Some(self.memory_threshold),
            "disk" => Some(self.disk_threshold),
            "latency" => Some(self.latency_threshold),
            "error_rate" => Some(self.error_rate_threshold),
            _ => None,
        }
    }

    /// Set threshold value for a metric
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn set_threshold(&mut self, metric: &str, value: f64) -> Result<(), String> {
        if value < 0.0 {
            return Err(ERROR_THRESHOLD_NEGATIVE.to_string());
        }

        match metric {
            "cpu" => {
                if value > 100.0 {
                    return Err(ERROR_CPU_THRESHOLD_EXCEED.to_string());
                }
                self.cpu_threshold = value;
            }
            "memory" => {
                if value > 100.0 {
                    return Err(ERROR_MEMORY_THRESHOLD_EXCEED.to_string());
                }
                self.memory_threshold = value;
            }
            "disk" => {
                if value > 100.0 {
                    return Err(ERROR_DISK_THRESHOLD_EXCEED.to_string());
                }
                self.disk_threshold = value;
            }
            "latency" => self.latency_threshold = value,
            "error_rate" => {
                if value > 100.0 {
                    return Err(ERROR_ERROR_RATE_EXCEED.to_string());
                }
                self.error_rate_threshold = value;
            }
            _ => return Err(format!("Unknown metric: {metric}")),
        }
        Ok(())
    }

    /// Validate threshold values
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.cpu_threshold < 0.0 || self.cpu_threshold > 100.0 {
            return Err(ERROR_CPU_THRESHOLD_RANGE);
        }

        if self.memory_threshold < 0.0 || self.memory_threshold > 100.0 {
            return Err(ERROR_MEMORY_THRESHOLD_RANGE);
        }

        if self.disk_threshold < 0.0 || self.disk_threshold > 100.0 {
            return Err(ERROR_DISK_THRESHOLD_RANGE);
        }

        if self.latency_threshold < 0.0 {
            return Err(ERROR_LATENCY_THRESHOLD_POSITIVE);
        }

        if self.error_rate_threshold < 0.0 || self.error_rate_threshold > 100.0 {
            return Err(ERROR_ERROR_RATE_RANGE);
        }
        Ok(())
    }
}
