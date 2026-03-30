// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

#![allow(clippy::redundant_pub_crate)]

//! Validation messages and shared literals for monitoring configuration.

// Log File Path Constants
pub(crate) const LOG_FILE_DEFAULT: &str = "./logs/nestgate.log";

// Slack Configuration Constants
pub(crate) const SLACK_CHANNEL_DEFAULT: &str = "general";
/// Default value for slack username default
pub(crate) const SLACK_USERNAME_DEFAULT: &str = "NestGate";
/// Slack Emoji Robot
pub(crate) const SLACK_EMOJI_ROBOT: &str = ":robot_face:";

// HTTP Method Constants
pub(crate) const HTTP_METHOD_POST: &str = "POST";

// Empty String Constant (Used 7 times)
pub(crate) const EMPTY_STRING: &str = "";

// Test constants - defined inline to avoid missing module dependency
#[cfg(test)]
pub(crate) const EXAMPLE_SENDER_EMAIL: &str = "noreply@example.com";
#[cfg(test)]
pub(crate) const EXAMPLE_TEST_EMAIL: &str = "test@example.com";
#[cfg(test)]
pub(crate) const EXAMPLE_SMTP_SERVER: &str = "smtp.example.com";
#[cfg(test)]
pub(crate) const EXAMPLE_SLACK_WEBHOOK: &str = "https://hooks.slack.com/services/example";
#[cfg(test)]
pub(crate) const EXAMPLE_WEBHOOK_URL: &str = "https://webhook.example.com";

// Validation Error Message Constants
pub(crate) const ERROR_METRICS_INTERVAL_ZERO: &str = "Metrics interval must be greater than 0";
/// Error Log File Empty
pub(crate) const ERROR_LOG_FILE_EMPTY: &str = "Log file path cannot be empty";
/// Error Log Rotation Size Zero
pub(crate) const ERROR_LOG_ROTATION_SIZE_ZERO: &str = "Log rotation size must be greater than 0";
/// Error Log Retention Zero
pub(crate) const ERROR_LOG_RETENTION_ZERO: &str = "Log retention days must be greater than 0";
/// Error Notification Required
pub(crate) const ERROR_NOTIFICATION_REQUIRED: &str =
    "At least one notification method must be configured when alerting is enabled";
/// Error Threshold Negative
pub(crate) const ERROR_THRESHOLD_NEGATIVE: &str = "Threshold value cannot be negative";
/// Error Cpu Threshold Range
pub(crate) const ERROR_CPU_THRESHOLD_RANGE: &str = "CPU threshold must be between 0 and 100";
/// Error Cpu Threshold Exceed
pub(crate) const ERROR_CPU_THRESHOLD_EXCEED: &str = "CPU threshold cannot exceed 100%";
/// Error Memory Threshold Range
pub(crate) const ERROR_MEMORY_THRESHOLD_RANGE: &str = "Memory threshold must be between 0 and 100";
/// Error Memory Threshold Exceed
pub(crate) const ERROR_MEMORY_THRESHOLD_EXCEED: &str = "Memory threshold cannot exceed 100%";
/// Error Disk Threshold Range
pub(crate) const ERROR_DISK_THRESHOLD_RANGE: &str = "Disk threshold must be between 0 and 100";
/// Error Disk Threshold Exceed
pub(crate) const ERROR_DISK_THRESHOLD_EXCEED: &str = "Disk threshold cannot exceed 100%";
/// Error Latency Threshold Positive
pub(crate) const ERROR_LATENCY_THRESHOLD_POSITIVE: &str = "Latency threshold must be positive";
/// Error Error Rate Range
pub(crate) const ERROR_ERROR_RATE_RANGE: &str = "Error rate threshold must be between 0 and 100";
/// Error Error Rate Exceed
pub(crate) const ERROR_ERROR_RATE_EXCEED: &str = "Error rate threshold cannot exceed 100%";
/// Error Smtp Server Empty
pub(crate) const ERROR_SMTP_SERVER_EMPTY: &str = "SMTP server cannot be empty";
/// Error Smtp Port Zero
pub(crate) const ERROR_SMTP_PORT_ZERO: &str = "SMTP port must be greater than 0";
/// Error Recipient Required
pub(crate) const ERROR_RECIPIENT_REQUIRED: &str =
    "At least one recipient address must be specified";
/// Error Slack Webhook Empty
pub(crate) const ERROR_SLACK_WEBHOOK_EMPTY: &str = "Slack webhook URL cannot be empty";
/// Error Slack Channel Empty
pub(crate) const ERROR_SLACK_CHANNEL_EMPTY: &str = "Slack channel cannot be empty";
/// Error Slack Username Empty
pub(crate) const ERROR_SLACK_USERNAME_EMPTY: &str = "Slack username cannot be empty";
/// Error Webhook Url Empty
pub(crate) const ERROR_WEBHOOK_URL_EMPTY: &str = "Webhook URL cannot be empty";
/// Error Http Method Empty
pub(crate) const ERROR_HTTP_METHOD_EMPTY: &str = "HTTP method cannot be empty";
/// Error Timeout Zero
pub(crate) const ERROR_TIMEOUT_ZERO: &str = "Timeout must be greater than 0";
/// Error Prometheus Port Zero
pub(crate) const ERROR_PROMETHEUS_PORT_ZERO: &str = "Prometheus port cannot be zero when enabled";
/// Error Email From Empty
pub(crate) const ERROR_EMAIL_FROM_EMPTY: &str = "Email from_endpoint cannot be empty";
