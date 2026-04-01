// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

use super::constants::EMPTY_STRING;
use super::{
    AlertThresholds, EmailConfig, MonitoringConfig, NotificationConfig, PrometheusConfig,
    SlackConfig, WebhookConfig,
};

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
        password: None,
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
    email
        .to_addresses
        .push(super::constants::EXAMPLE_TEST_EMAIL.to_string());
    email.smtp_server = super::constants::EXAMPLE_SMTP_SERVER.to_string();
    email.from_endpoint = super::constants::EXAMPLE_SENDER_EMAIL.to_string();
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
    slack.webhook_url = Some(super::constants::EXAMPLE_SLACK_WEBHOOK.to_string());
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
    webhook.url = super::constants::EXAMPLE_WEBHOOK_URL.to_string();
    assert!(webhook.validate().is_ok());

    // Zero timeout should fail
    webhook.timeout = 0;
    assert!(webhook.validate().is_err());
}
