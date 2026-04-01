// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Email, Slack, and webhook notification channels for alerting.

#![allow(clippy::wildcard_imports)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::constants::*;

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

    /// SMTP password (set via `NESTGATE_SMTP_PASSWORD` or config; never a hardcoded default)
    #[serde(default)]
    pub password: Option<String>,

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
    /// Slack webhook URL (set via `NESTGATE_SLACK_WEBHOOK_URL` or config; no default URL)
    #[serde(default)]
    pub webhook_url: Option<String>,
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

impl Default for EmailConfig {
    /// Returns the default instance
    fn default() -> Self {
        let password = std::env::var("NESTGATE_SMTP_PASSWORD")
            .ok()
            .filter(|s| !s.is_empty());
        Self {
            smtp_server: "smtp.example.com".to_string(),
            smtp_port: 587,
            username: "user@example.com".to_string(),
            password,
            from_endpoint: "noreply@example.com".to_string(),
            to_addresses: vec!["admin@example.com".to_string()],
            enable_tls: true,
        }
    }
}

impl Default for SlackConfig {
    /// Returns the default instance
    fn default() -> Self {
        let webhook_url = std::env::var("NESTGATE_SLACK_WEBHOOK_URL")
            .ok()
            .filter(|s| !s.is_empty());
        Self {
            webhook_url,
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
    pub fn validate(&self) -> Result<(), &'static str> {
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
    pub const fn validate(&self) -> Result<(), &'static str> {
        if self.smtp_server.is_empty() {
            return Err(ERROR_SMTP_SERVER_EMPTY);
        }

        if self.smtp_port == 0 {
            return Err(ERROR_SMTP_PORT_ZERO);
        }

        if self.from_endpoint.is_empty() {
            return Err(ERROR_EMAIL_FROM_EMPTY);
        }

        if self.to_addresses.is_empty() {
            return Err(ERROR_RECIPIENT_REQUIRED);
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
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.webhook_url.as_deref().is_none_or(str::is_empty) {
            return Err(ERROR_SLACK_WEBHOOK_EMPTY);
        }

        if self.channel.is_empty() {
            return Err(ERROR_SLACK_CHANNEL_EMPTY);
        }

        if self.username.is_empty() {
            return Err(ERROR_SLACK_USERNAME_EMPTY);
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
    pub const fn validate(&self) -> Result<(), &'static str> {
        if self.url.is_empty() {
            return Err(ERROR_WEBHOOK_URL_EMPTY);
        }

        if self.method.is_empty() {
            return Err(ERROR_HTTP_METHOD_EMPTY);
        }

        if self.timeout == 0 {
            return Err(ERROR_TIMEOUT_ZERO);
        }
        Ok(())
    }
}
