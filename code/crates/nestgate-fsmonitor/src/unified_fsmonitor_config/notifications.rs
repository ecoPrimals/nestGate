// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

/// Notification and alerting configuration - extracted from monolithic config
/// Handles all notification channels, formatting, throttling, and routing
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
/// Notification settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSettings {
    /// Enable notifications
    pub enabled: bool,
    /// Enable console output
    pub enable_console_output: bool,
    /// Notification channels
    pub channels: Vec<NotificationChannel>,
    /// Notification formatting
    pub formatting: NotificationFormattingSettings,
    /// Notification throttling
    pub throttling: NotificationThrottlingSettings,
    /// Notification routing
    pub routing: NotificationRoutingSettings,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationChannel {
    /// Channel name
    pub name: String,
    /// Channel type (email, slack, webhook, etc.)
    pub channel_type: String,
    /// Channel enabled
    pub enabled: bool,
    /// Channel configuration
    pub config: HashMap<String, serde_json::Value>,
    /// Priority threshold
    pub priority_threshold: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationFormattingSettings {
    /// Enable rich formatting
    pub rich_formatting: bool,
    /// Include timestamps
    pub include_timestamps: bool,
    /// Include metadata
    pub include_metadata: bool,
    /// Template configurations
    pub templates: HashMap<String, String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationThrottlingSettings {
    /// Enable throttling
    pub enabled: bool,
    /// Throttle window duration
    pub window_duration: Duration,
    /// Maximum notifications per window
    pub max_notifications_per_window: u32,
    /// Burst limit
    pub burst_limit: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NotificationRoutingSettings {
    /// Enable routing
    pub enabled: bool,
    /// Routing rules
    pub rules: Vec<RoutingRule>,
    /// Default route
    pub default_route: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingRule {
    /// Rule name
    pub name: String,
    /// Rule condition
    pub condition: String,
    /// Target channels
    pub target_channels: Vec<String>,
    /// Rule priority
    pub priority: u32,
}
impl Default for NotificationSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            enable_console_output: false,
            channels: Vec::new(),
            formatting: NotificationFormattingSettings::default(),
            throttling: NotificationThrottlingSettings::default(),
            routing: NotificationRoutingSettings::default(),
        }
    }
}

impl Default for NotificationFormattingSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            rich_formatting: true,
            include_timestamps: true,
            include_metadata: true,
            templates: HashMap::new(),
        }
    }
}

impl Default for NotificationThrottlingSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            window_duration: Duration::from_secs(5 * 60),
            max_notifications_per_window: 10,
            burst_limit: 3,
        }
    }
}
