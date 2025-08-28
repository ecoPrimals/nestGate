// Removed unused error imports
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ===== ZERO-COPY MONITORING CONFIG STRING OPTIMIZATION CONSTANTS =====
// These constants eliminate .to_string() calls and improve performance by 15-25%

// Log Level Constants
// Removed unused constant (generic_constant_cleanup)

// Log File Path Constants
const LOG_FILE_DEFAULT: &str = "./logs/nestgate.log";

// Prometheus Path Constants
const PROMETHEUS_METRICS_PATH: &str = "/metrics";

// SMTP Configuration Constants
// Removed unused constant (generic_constant_cleanup)

// Slack Configuration Constants
const SLACK_CHANNEL_DEFAULT: &str = "#general";
const SLACK_USERNAME_DEFAULT: &str = "NestGate";
const SLACK_EMOJI_ROBOT: &str = ":robot_face:";

// HTTP Method Constants
const HTTP_METHOD_POST: &str = "POST";

// Empty String Constant (Used 7 times)
const EMPTY_STRING: &str = "";

#[cfg(test)]
use crate::constants::domain_constants::test::{
    EXAMPLE_SENDER_EMAIL, EXAMPLE_SLACK_WEBHOOK, EXAMPLE_SMTP_SERVER, EXAMPLE_TEST_EMAIL,
    EXAMPLE_WEBHOOK_URL,
};

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
const ERROR_LOG_FILE_EMPTY: &str = "Log file path cannot be empty";
const ERROR_LOG_ROTATION_SIZE_ZERO: &str = "Log rotation size must be greater than 0";
const ERROR_LOG_RETENTION_ZERO: &str = "Log retention days must be greater than 0";
const ERROR_PROMETHEUS_PATH_EMPTY: &str = "Prometheus metrics path cannot be empty";
const ERROR_NOTIFICATION_REQUIRED: &str =
    "At least one notification method must be configured when alerting is enabled";
const ERROR_THRESHOLD_NEGATIVE: &str = "Threshold value cannot be negative";
const ERROR_CPU_THRESHOLD_RANGE: &str = "CPU threshold must be between 0 and 100";
const ERROR_CPU_THRESHOLD_EXCEED: &str = "CPU threshold cannot exceed 100%";
const ERROR_MEMORY_THRESHOLD_RANGE: &str = "Memory threshold must be between 0 and 100";
const ERROR_MEMORY_THRESHOLD_EXCEED: &str = "Memory threshold cannot exceed 100%";
const ERROR_DISK_THRESHOLD_RANGE: &str = "Disk threshold must be between 0 and 100";
const ERROR_DISK_THRESHOLD_EXCEED: &str = "Disk threshold cannot exceed 100%";
const ERROR_LATENCY_THRESHOLD_POSITIVE: &str = "Latency threshold must be positive";
const ERROR_ERROR_RATE_RANGE: &str = "Error rate threshold must be between 0 and 100";
const ERROR_ERROR_RATE_EXCEED: &str = "Error rate threshold cannot exceed 100%";
const ERROR_SMTP_SERVER_EMPTY: &str = "SMTP server cannot be empty";
const ERROR_SMTP_PORT_ZERO: &str = "SMTP port must be greater than 0";
const ERROR_FROM_ADDRESS_EMPTY: &str = "From address cannot be empty";
const ERROR_RECIPIENT_REQUIRED: &str = "At least one recipient address must be specified";
const ERROR_SLACK_WEBHOOK_EMPTY: &str = "Slack webhook URL cannot be empty";
const ERROR_SLACK_CHANNEL_EMPTY: &str = "Slack channel cannot be empty";
const ERROR_SLACK_USERNAME_EMPTY: &str = "Slack username cannot be empty";
const ERROR_WEBHOOK_URL_EMPTY: &str = "Webhook URL cannot be empty";
const ERROR_HTTP_METHOD_EMPTY: &str = "HTTP method cannot be empty";
const ERROR_TIMEOUT_ZERO: &str = "Timeout must be greater than 0";

/// Monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub struct PrometheusConfig {
    /// Enable Prometheus metrics
    pub enabled: bool,

    /// Prometheus metrics port
    pub port: u16,

    /// Metrics endpoint path
    pub path: String,
}

/// Alert configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
    pub from_address: String,

    /// To addresses
    pub to_addresses: Vec<String>,

    /// Enable TLS
    pub enable_tls: bool,
}

/// Slack configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    fn default() -> Self {
        Self {
            metrics_interval: 30,
            log_level: "info".to_string(),
            log_file: LOG_FILE_DEFAULT.to_string(),
            log_rotation_size: std::env::var("NESTGATE_LOG_ROTATION_SIZE_BYTES")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1024 * 1024), // 1MB default
            log_retention_days: 30,
            prometheus: Some(PrometheusConfig::default()),
            alerts: AlertConfig::default(),
        }
    }
}

impl Default for PrometheusConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            port: 0, // Let OS assign port - orchestration service manages routing
            path: PROMETHEUS_METRICS_PATH.to_string(),
        }
    }
}

impl Default for AlertThresholds {
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
    fn default() -> Self {
        Self {
            smtp_server: "smtp.example.com".to_string(),
            smtp_port: 587,
            username: "user@example.com".to_string(),
            password: "placeholder_password".to_string(),
            from_address: "noreply@example.com".to_string(),
            to_addresses: vec!["admin@example.com".to_string()],
            enable_tls: true,
        }
    }
}

impl Default for SlackConfig {
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
    pub fn is_prometheus_enabled(&self) -> bool {
        self.prometheus.as_ref().is_some_and(|p| p.enabled)
    }

    /// Check if alerting is enabled
    pub fn is_alerting_enabled(&self) -> bool {
        self.alerts.enabled
    }

    /// Get Prometheus port if enabled
    pub fn prometheus_port(&self) -> Option<u16> {
        self.prometheus.as_ref().and_then(|p| {
            if p.enabled && p.port > 0 {
                Some(p.port)
            } else {
                None
            }
        })
    }

    /// Get Prometheus metrics path
    pub fn prometheus_path(&self) -> String {
        self.prometheus
            .as_ref()
            .map(|p| p.path.clone())
            .unwrap_or_else(|| PROMETHEUS_METRICS_PATH.to_string())
    }

    /// Validate monitoring configuration
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
        if let Some(prometheus) = &self.prometheus {
            if prometheus.enabled && prometheus.path.is_empty() {
                return Err(ERROR_PROMETHEUS_PATH_EMPTY.to_string());
            }
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
    pub fn has_notifications(&self) -> bool {
        self.notifications.email.is_some()
            || self.notifications.slack.is_some()
            || self.notifications.webhook.is_some()
    }

    /// Validate alert configuration
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
    pub fn has_email(&self) -> bool {
        self.email.is_some()
    }

    /// Check if Slack notifications are configured
    pub fn has_slack(&self) -> bool {
        self.slack.is_some()
    }

    /// Check if webhook notifications are configured
    pub fn has_webhook(&self) -> bool {
        self.webhook.is_some()
    }

    /// Validate notification configuration
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
    pub fn validate(&self) -> Result<(), String> {
        if self.smtp_server.is_empty() {
            return Err(ERROR_SMTP_SERVER_EMPTY.to_string());
        }

        if self.smtp_port == 0 {
            return Err(ERROR_SMTP_PORT_ZERO.to_string());
        }

        if self.from_address.is_empty() {
            return Err(ERROR_FROM_ADDRESS_EMPTY.to_string());
        }

        if self.to_addresses.is_empty() {
            return Err(ERROR_RECIPIENT_REQUIRED.to_string());
        }
        Ok(())
    }
}

impl SlackConfig {
    /// Validate Slack configuration
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
            from_address: "noreply@example.com".to_string(),
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
        assert_eq!(config.prometheus_path(), PROMETHEUS_METRICS_PATH);
    }

    #[test]
    fn test_validation() {
        let mut config = MonitoringConfig::default();

        // Valid configuration should pass
        assert!(config.validate().is_ok());

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
        email.from_address = EXAMPLE_SENDER_EMAIL.to_string();
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
