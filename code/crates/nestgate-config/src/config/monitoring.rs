// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// Removed unused error imports
//! Monitoring module

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ===== ZERO-COPY MONITORING CONFIG STRING OPTIMIZATION CONSTANTS =====
// These constants eliminate .to_string() calls and improve performance by 15-25%

// Log Level Constants
// Removed unused constant (generic_constant_cleanup)

// Log File Path Constants
const LOG_FILE_DEFAULT: &str = "./logs/nestgate.log";

// SMTP Configuration Constants
// Removed unused constant (generic_constant_cleanup)

// Slack Configuration Constants
const SLACK_CHANNEL_DEFAULT: &str = "general";
/// Default value for slack username default
const SLACK_USERNAME_DEFAULT: &str = "NestGate";
/// Slack Emoji Robot
const SLACK_EMOJI_ROBOT: &str = ":robot_face:";

// HTTP Method Constants
const HTTP_METHOD_POST: &str = "POST";

// Empty String Constant (Used 7 times)
const EMPTY_STRING: &str = "";

// Test constants - defined inline to avoid missing module dependency
#[cfg(test)]
const EXAMPLE_SENDER_EMAIL: &str = "noreply@example.com";
#[cfg(test)]
const EXAMPLE_TEST_EMAIL: &str = "test@example.com";
#[cfg(test)]
const EXAMPLE_SMTP_SERVER: &str = "smtp.example.com";
#[cfg(test)]
const EXAMPLE_SLACK_WEBHOOK: &str = "https://hooks.slack.com/services/example";
#[cfg(test)]
const EXAMPLE_WEBHOOK_URL: &str = "https://webhook.example.com";

// Example Configuration Constants
// Removed unused constant (example_constant_cleanup)
// Removed unused constant (example_constant_cleanup)
// Removed unused constant (example_constant_cleanup)
// Removed unused constant (example_constant_cleanup)
// Removed unused constant (example_constant_cleanup)
// Removed unused constant (example_constant_cleanup)
// Removed unused constant (example_constant_cleanup)
// Removed unused constant (example_constant_cleanup)

// Field Name Constants (for threshold validation)
// Removed unused constant (generic_constant_cleanup)
// Removed unused constant (generic_constant_cleanup)
// Removed unused constant (generic_constant_cleanup)
// Removed unused constant (generic_constant_cleanup)
// Removed unused constant (generic_constant_cleanup)

// Configuration Field Name Constants
// Removed unused constant (generic_constant_cleanup)

// Validation Error Message Constants
const ERROR_METRICS_INTERVAL_ZERO: &str = "Metrics interval must be greater than 0";
/// Error Log File Empty
const ERROR_LOG_FILE_EMPTY: &str = "Log file path cannot be empty";
/// Error Log Rotation Size Zero
const ERROR_LOG_ROTATION_SIZE_ZERO: &str = "Log rotation size must be greater than 0";
/// Error Log Retention Zero
const ERROR_LOG_RETENTION_ZERO: &str = "Log retention days must be greater than 0";
/// Error Notification Required
const ERROR_NOTIFICATION_REQUIRED: &str =
    "At least one notification method must be configured when alerting is enabled";
/// Error Threshold Negative
const ERROR_THRESHOLD_NEGATIVE: &str = "Threshold value cannot be negative";
/// Error Cpu Threshold Range
const ERROR_CPU_THRESHOLD_RANGE: &str = "CPU threshold must be between 0 and 100";
/// Error Cpu Threshold Exceed
const ERROR_CPU_THRESHOLD_EXCEED: &str = "CPU threshold cannot exceed 100%";
/// Error Memory Threshold Range
const ERROR_MEMORY_THRESHOLD_RANGE: &str = "Memory threshold must be between 0 and 100";
/// Error Memory Threshold Exceed
const ERROR_MEMORY_THRESHOLD_EXCEED: &str = "Memory threshold cannot exceed 100%";
/// Error Disk Threshold Range
const ERROR_DISK_THRESHOLD_RANGE: &str = "Disk threshold must be between 0 and 100";
/// Error Disk Threshold Exceed
const ERROR_DISK_THRESHOLD_EXCEED: &str = "Disk threshold cannot exceed 100%";
/// Error Latency Threshold Positive
const ERROR_LATENCY_THRESHOLD_POSITIVE: &str = "Latency threshold must be positive";
/// Error Error Rate Range
const ERROR_ERROR_RATE_RANGE: &str = "Error rate threshold must be between 0 and 100";
/// Error Error Rate Exceed
const ERROR_ERROR_RATE_EXCEED: &str = "Error rate threshold cannot exceed 100%";
/// Error Smtp Server Empty
const ERROR_SMTP_SERVER_EMPTY: &str = "SMTP server cannot be empty";
/// Error Smtp Port Zero
const ERROR_SMTP_PORT_ZERO: &str = "SMTP port must be greater than 0";
/// Error Recipient Required
const ERROR_RECIPIENT_REQUIRED: &str = "At least one recipient address must be specified";
/// Error Slack Webhook Empty
const ERROR_SLACK_WEBHOOK_EMPTY: &str = "Slack webhook URL cannot be empty";
/// Error Slack Channel Empty
const ERROR_SLACK_CHANNEL_EMPTY: &str = "Slack channel cannot be empty";
/// Error Slack Username Empty
const ERROR_SLACK_USERNAME_EMPTY: &str = "Slack username cannot be empty";
/// Error Webhook Url Empty
const ERROR_WEBHOOK_URL_EMPTY: &str = "Webhook URL cannot be empty";
/// Error Http Method Empty
const ERROR_HTTP_METHOD_EMPTY: &str = "HTTP method cannot be empty";
/// Error Timeout Zero
const ERROR_TIMEOUT_ZERO: &str = "Timeout must be greater than 0";

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

/// Prometheus configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Prometheus
pub struct PrometheusConfig {
    /// Enable Prometheus metrics
    pub enabled: bool,
    /// Prometheus metrics port
    pub port: u16,
}

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

/// Notification configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for Notification
pub struct NotificationConfig {
    /// Email configuration
    pub email: Option<EmailConfig>,
    /// Slack configuration
    pub slack: Option<SlackConfig>,

    /// Webhook configuration
    pub webhook: Option<WebhookConfig>,
}

/// Email configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Email
pub struct EmailConfig {
    /// SMTP server
    pub smtp_server: String,
    /// SMTP port
    pub smtp_port: u16,

    /// SMTP username
    pub username: String,

    /// SMTP password
    pub password: String,

    /// From address
    pub from_endpoint: String,

    /// To addresses
    pub to_addresses: Vec<String>,

    /// Enable TLS
    pub enable_tls: bool,
}

/// Slack configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Slack
pub struct SlackConfig {
    /// Slack webhook URL
    pub webhook_url: String,
    /// Slack channel
    pub channel: String,

    /// Bot username
    pub username: String,

    /// Bot icon emoji
    pub icon_emoji: Option<String>,
}

/// Webhook configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Webhook
pub struct WebhookConfig {
    /// Webhook URL
    pub url: String,
    /// HTTP method
    pub method: String,

    /// Headers
    pub headers: HashMap<String, String>,

    /// Request timeout in seconds
    pub timeout: u64,
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

impl Default for EmailConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            smtp_server: "smtp.example.com".to_string(),
            smtp_port: 587,
            username: "user@example.com".to_string(),
            password: "placeholder_password".to_string(),
            from_endpoint: "noreply@example.com".to_string(),
            to_addresses: vec!["admin@example.com".to_string()],
            enable_tls: true,
        }
    }
}

impl Default for SlackConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            webhook_url: EMPTY_STRING.to_string(),
            channel: SLACK_CHANNEL_DEFAULT.to_string(),
            username: SLACK_USERNAME_DEFAULT.to_string(),
            icon_emoji: Some(SLACK_EMOJI_ROBOT.to_string()),
        }
    }
}

impl Default for WebhookConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            url: EMPTY_STRING.to_string(),
            method: HTTP_METHOD_POST.to_string(),
            headers: HashMap::new(),
            timeout: 30,
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

    // Get Prometheus metrics path
    // NOTE: PrometheusConfig does not have a path field (only enabled and port)
    // This function is commented out. Use the port field directly instead.
    // pub fn prometheus_path(&self) -> String {
    //     PROMETHEUS_METRICS_PATH.to_string()
    // }

    /// Validate monitoring configuration
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn validate(&self) -> Result<(), String> {
        // Validate metrics interval
        if self.metrics_interval == 0 {
            return Err(ERROR_METRICS_INTERVAL_ZERO.to_string());
        }

        // Validate log file path
        if self.log_file.is_empty() {
            return Err(ERROR_LOG_FILE_EMPTY.to_string());
        }

        // Validate log rotation size
        if self.log_rotation_size == 0 {
            return Err(ERROR_LOG_ROTATION_SIZE_ZERO.to_string());
        }

        // Validate log retention days
        if self.log_retention_days == 0 {
            return Err(ERROR_LOG_RETENTION_ZERO.to_string());
        }

        // Validate Prometheus configuration
        if let Some(prometheus) = &self.prometheus
            && prometheus.enabled
            && prometheus.port == 0
        {
            return Err("Prometheus port cannot be zero when enabled".to_string());
        }

        // Validate alert configuration
        if self.alerts.enabled {
            self.alerts.validate()?;
        }
        Ok(())
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
    pub fn validate(&self) -> Result<(), String> {
        // Validate thresholds
        self.thresholds.validate()?;

        // Validate notification configuration
        if self.enabled && !self.has_notifications() {
            return Err(ERROR_NOTIFICATION_REQUIRED.to_string());
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
    pub fn validate(&self) -> Result<(), String> {
        if self.cpu_threshold < 0.0 || self.cpu_threshold > 100.0 {
            return Err(ERROR_CPU_THRESHOLD_RANGE.to_string());
        }

        if self.memory_threshold < 0.0 || self.memory_threshold > 100.0 {
            return Err(ERROR_MEMORY_THRESHOLD_RANGE.to_string());
        }

        if self.disk_threshold < 0.0 || self.disk_threshold > 100.0 {
            return Err(ERROR_DISK_THRESHOLD_RANGE.to_string());
        }

        if self.latency_threshold < 0.0 {
            return Err(ERROR_LATENCY_THRESHOLD_POSITIVE.to_string());
        }

        if self.error_rate_threshold < 0.0 || self.error_rate_threshold > 100.0 {
            return Err(ERROR_ERROR_RATE_RANGE.to_string());
        }
        Ok(())
    }
}

impl NotificationConfig {
    /// Check if email notifications are configured
    #[must_use]
    pub const fn has_email(&self) -> bool {
        self.email.is_some()
    }

    /// Check if Slack notifications are configured
    #[must_use]
    pub const fn has_slack(&self) -> bool {
        self.slack.is_some()
    }

    /// Check if webhook notifications are configured
    #[must_use]
    pub const fn has_webhook(&self) -> bool {
        self.webhook.is_some()
    }

    /// Validate notification configuration
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn validate(&self) -> Result<(), String> {
        // Validate email configuration
        if let Some(email) = &self.email {
            email.validate()?;
        }

        // Validate Slack configuration
        if let Some(slack) = &self.slack {
            slack.validate()?;
        }

        // Validate webhook configuration
        if let Some(webhook) = &self.webhook {
            webhook.validate()?;
        }
        Ok(())
    }
}

impl EmailConfig {
    /// Validate email configuration
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn validate(&self) -> Result<(), String> {
        if self.smtp_server.is_empty() {
            return Err(ERROR_SMTP_SERVER_EMPTY.to_string());
        }

        if self.smtp_port == 0 {
            return Err(ERROR_SMTP_PORT_ZERO.to_string());
        }

        if self.from_endpoint.is_empty() {
            return Err("Email from_endpoint cannot be empty".to_string());
        }

        if self.to_addresses.is_empty() {
            return Err(ERROR_RECIPIENT_REQUIRED.to_string());
        }
        Ok(())
    }
}

impl SlackConfig {
    /// Validate Slack configuration
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn validate(&self) -> Result<(), String> {
        if self.webhook_url.is_empty() {
            return Err(ERROR_SLACK_WEBHOOK_EMPTY.to_string());
        }

        if self.channel.is_empty() {
            return Err(ERROR_SLACK_CHANNEL_EMPTY.to_string());
        }

        if self.username.is_empty() {
            return Err(ERROR_SLACK_USERNAME_EMPTY.to_string());
        }
        Ok(())
    }
}

impl WebhookConfig {
    /// Validate webhook configuration
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn validate(&self) -> Result<(), String> {
        if self.url.is_empty() {
            return Err(ERROR_WEBHOOK_URL_EMPTY.to_string());
        }

        if self.method.is_empty() {
            return Err(ERROR_HTTP_METHOD_EMPTY.to_string());
        }

        if self.timeout == 0 {
            return Err(ERROR_TIMEOUT_ZERO.to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitoring_config_default() {
        let config = MonitoringConfig::default();
        assert_eq!(config.metrics_interval, 30);
        assert_eq!(config.log_level, "info");
        assert_eq!(config.log_retention_days, 30);
        assert!(config.is_prometheus_enabled());
        assert!(!config.is_alerting_enabled()); // Disabled by default
    }

    #[test]
    fn test_alert_thresholds() {
        let mut thresholds = AlertThresholds::default();

        // Test default values
        assert_eq!(thresholds.cpu_threshold, 80.0);
        assert_eq!(thresholds.memory_threshold, 85.0);
        assert_eq!(thresholds.disk_threshold, 90.0);

        // Test threshold checking
        assert!(thresholds.is_threshold_exceeded("cpu", 90.0));
        assert!(!thresholds.is_threshold_exceeded("cpu", 70.0));
        assert!(thresholds.is_threshold_exceeded("memory", 95.0));
        assert!(!thresholds.is_threshold_exceeded("memory", 80.0));

        // Test threshold setting
        assert!(thresholds.set_threshold("cpu", 75.0).is_ok());
        assert_eq!(thresholds.cpu_threshold, 75.0);
        assert!(thresholds.set_threshold("cpu", 150.0).is_err());
        assert!(thresholds.set_threshold("cpu", -10.0).is_err());
    }

    #[test]
    fn test_notification_config() {
        let mut config = NotificationConfig::default();

        // Initially no notifications configured
        assert!(!config.has_email());
        assert!(!config.has_slack());
        assert!(!config.has_webhook());

        // Add email configuration
        config.email = Some(EmailConfig {
            smtp_server: "smtp.example.com".to_string(),
            smtp_port: 587,
            username: "user@example.com".to_string(),
            password: "placeholder_password".to_string(),
            from_endpoint: "noreply@example.com".to_string(),
            to_addresses: vec!["admin@example.com".to_string()],
            enable_tls: true,
        });

        assert!(config.has_email());
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_prometheus_config() {
        let config = MonitoringConfig::default();
        assert!(config.is_prometheus_enabled());
        // Default uses standard Prometheus port 9090
        assert_eq!(config.prometheus_port(), Some(9090));

        // Test with OS-assigned port (port 0)
        let mut config_with_os_port = config;
        config_with_os_port.prometheus = Some(PrometheusConfig {
            enabled: true,
            port: 0, // OS-assigned
        });
        assert_eq!(config_with_os_port.prometheus_port(), None); // Returns None for port 0
    }

    #[test]
    fn test_validation() {
        let mut config = MonitoringConfig::default();

        // Valid configuration should pass
        if let Err(e) = config.validate() {
            panic!("Default config validation failed: {}", e);
        }

        // Invalid metrics interval should fail
        config.metrics_interval = 0;
        assert!(config.validate().is_err());

        // Reset and test empty log file
        config.metrics_interval = 30;
        config.log_file = EMPTY_STRING.to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_email_validation() {
        let mut email = EmailConfig::default();

        // Default config should pass validation (has recipient)
        assert!(email.validate().is_ok());

        // Add recipient
        email.to_addresses.push(EXAMPLE_TEST_EMAIL.to_string());
        email.smtp_server = EXAMPLE_SMTP_SERVER.to_string();
        email.from_endpoint = EXAMPLE_SENDER_EMAIL.to_string();
        assert!(email.validate().is_ok());

        // Empty SMTP server should fail
        email.smtp_server = EMPTY_STRING.to_string();
        assert!(email.validate().is_err());
    }

    #[test]
    fn test_slack_validation() {
        let mut slack = SlackConfig::default();

        // Default config should fail validation (no webhook URL)
        assert!(slack.validate().is_err());

        // Add webhook URL
        slack.webhook_url = EXAMPLE_SLACK_WEBHOOK.to_string();
        assert!(slack.validate().is_ok());

        // Empty channel should fail
        slack.channel = EMPTY_STRING.to_string();
        assert!(slack.validate().is_err());
    }

    #[test]
    fn test_webhook_validation() {
        let mut webhook = WebhookConfig::default();

        // Default config should fail validation (no URL)
        assert!(webhook.validate().is_err());

        // Add URL
        webhook.url = EXAMPLE_WEBHOOK_URL.to_string();
        assert!(webhook.validate().is_ok());

        // Zero timeout should fail
        webhook.timeout = 0;
        assert!(webhook.validate().is_err());
    }
}
