//! Alert Notification Channels

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Alert notification channel types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertChannel {
    /// Email notification
    Email {
        /// Email addresses to notify
        addresses: Vec<String>,
        /// SMTP configuration
        smtp: SmtpConfig,
    },
    /// Slack notification
    Slack {
        /// Slack webhook URL
        webhook_url: String,
        /// Channel to post to
        channel: String,
    },
    /// Discord notification
    Discord {
        /// Discord webhook URL
        webhook_url: String,
    },
    /// Webhook notification
    Webhook {
        /// Webhook URL
        url: String,
        /// HTTP headers to include
        headers: std::collections::HashMap<String, String>,
    },
    /// Log-based notification (for testing/debugging)
    Log {
        /// Log level
        level: String,
    },
}

/// SMTP server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmtpConfig {
    /// SMTP server hostname
    pub host: String,
    /// SMTP server port
    pub port: u16,
    /// Username for authentication
    pub username: String,
    /// Password for authentication (should be encrypted in production)
    pub password: String,
    /// Use TLS encryption
    pub use_tls: bool,
}

/// Record of a notification sent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationRecord {
    /// Alert ID that triggered the notification
    pub alert_id: String,
    /// Channel used for notification
    pub channel: String,
    /// When the notification was sent
    pub sent_at: SystemTime,
    /// Whether the notification was successful
    pub success: bool,
    /// Error message if notification failed
    pub error: Option<String>,
}
